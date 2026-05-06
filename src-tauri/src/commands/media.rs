use tauri::{ AppHandle, Emitter, Manager };
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_shell::{ ShellExt, process::CommandEvent };
use std::{ collections::HashMap, fs, path::Path, thread, time::Duration };
use serde::{ Deserialize, Serialize };
use serde_json::Value;
use super::ffmpeg_builder::FfmpegBuilder;

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct SubtitleChanges {
    pub default: bool,
    pub forced: bool,
    pub title: String,
    pub language: String,
    pub isDeleted: bool,
    pub isImported: bool,
    pub path: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct AudioChanges {
    pub default: bool,
    pub forced: bool,
    pub title: String,
    pub language: String,
    pub isDeleted: bool,
    pub isImported: bool,
    pub path: Option<String>,
    pub bitRate: Option<String>,
    pub bitDepth: Option<f64>,
    pub channels: Option<f64>,
    pub newChannels: Option<f64>,
    pub codecName: Option<String>,
    pub sampleRate: Option<String>,
    pub channelMap: Option<HashMap<String, Value>>,
    pub channelVolumes: Option<HashMap<String, Value>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct VideoChanges {
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub codecName: Option<String>,
    pub aspectRatio: Option<String>,
    pub pixelFormat: Option<String>,
    pub fieldOrder: Option<String>,
    pub profile: Option<String>,
    pub format: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct PendingChanges {
    pub video: Option<VideoChanges>,
    pub audio: Option<Vec<AudioChanges>>,
    pub subtitle: Option<Vec<SubtitleChanges>>,
}

struct MediaInfo {
    muxer: String,
    duration: f64,
}

#[tauri::command]
pub async fn get_media_streams(app: AppHandle, path: String) -> Result<Value, String> {
    let output = app
        .shell()
        .sidecar("ffprobe")
        .map_err(|e| {
            let msg = format!("Failed to find ffprobe: {}", e);
            let _ = app.emit("backend-error", &msg);
            msg
        })?
        .args([
            "-v",
            "error",
            "-show_entries",
            concat!(
                "stream=index,codec_name,codec_type,bit_rate,channels,bits_per_sample,sample_rate,",
                "width,height,display_aspect_ratio,avg_frame_rate,r_frame_rate,",
                "profile,level,pix_fmt,field_order,",
                "color_range,color_space,color_primaries,color_transfer",
                ":stream_tags=language,title",
                ":disposition=default,forced"
            ),
            "-of",
            "json",
            &path,
        ])
        .output().await
        .map_err(|e| {
            let msg = format!("Failed to execute ffprobe: {}", e);
            let _ = app.emit("backend-error", &msg);
            msg
        })?;

    serde_json::from_slice(&output.stdout).map_err(|e| {
        let msg = format!("Failed to parse ffprobe: {}", e);
        let _ = app.emit("backend-error", &msg);
        msg
    })
}

#[tauri::command]
pub async fn get_audio_path(
    app: AppHandle,
    input_path: String,
    stream_index: usize
) -> Result<String, String> {
    let cache_dir = app
        .path()
        .app_cache_dir()
        .map_err(|e| {
            let msg = format!("Failed to get cache dir: {}", e);
            let _ = app.emit("backend-error", &msg);
            msg
        })?;

    if !cache_dir.exists() {
        std::fs::create_dir_all(&cache_dir).map_err(|e| {
            let msg = format!("Failed to create cache dir: {}", e);
            let _ = app.emit("backend-error", &msg);
            msg
        })?;
    }

    let output_path = cache_dir.join("temp_visualizer.wav");
    let output_path_str = output_path
        .to_str()
        .ok_or_else(|| {
            let msg = "Invalid cache path".to_string();
            let _ = app.emit("backend-error", &msg);
            msg
        })?
        .to_string();

    let args = vec![
        "-i".to_string(),
        input_path,
        "-map".to_string(),
        format!("0:a:{}", stream_index),
        "-f".to_string(),
        "wav".to_string(),
        "-acodec".to_string(),
        "pcm_s16le".to_string(),
        "-ar".to_string(),
        "22050".to_string(),
        "-progress".to_string(),
        "pipe:1".to_string(),
        "-nostats".to_string(),
        "-y".to_string(),
        output_path_str.clone()
    ];

    let _ = app.emit("ffmpeg-log", format!("> ffmpeg {}", args.join(" ")));

    let (mut rx, _child) = app
        .shell()
        .sidecar("ffmpeg")
        .map_err(|e| {
            let msg = format!("FFmpeg sidecar error: {}", e);
            let _ = app.emit("backend-error", &msg);
            msg
        })?
        .args(args)
        .spawn()
        .map_err(|e| {
            let msg = format!("Failed to spawn FFmpeg: {}", e);
            let _ = app.emit("backend-error", &msg);
            msg
        })?;

    while let Some(event) = rx.recv().await {
        if let CommandEvent::Stderr(line) = event {
            let out = String::from_utf8_lossy(&line);
            let _ = app.emit("ffmpeg-log", out.to_string());
        }
    }

    Ok(output_path_str)
}

async fn get_media_info(app: &AppHandle, path: &str) -> Result<MediaInfo, String> {
    let output = app
        .shell()
        .sidecar("ffprobe")
        .map_err(|e| e.to_string())?
        .args([
            "-v",
            "error",
            "-show_entries",
            "format=format_name,duration",
            "-of",
            "default=noprint_wrappers=1",
            path,
        ])
        .output().await
        .map_err(|e| e.to_string())?;

    let res = String::from_utf8_lossy(&output.stdout);
    let mut info = MediaInfo { muxer: "matroska".into(), duration: 0.0 };

    for line in res.lines() {
        if let Some(val) = line.split('=').last() {
            if line.starts_with("format_name=") {
                info.muxer = val.split(',').next().unwrap_or("matroska").to_string();
            } else if line.starts_with("duration=") {
                info.duration = val.parse().unwrap_or(0.0);
            }
        }
    }
    Ok(info)
}

fn build_ffmpeg_args(
    file_path: &str,
    changes: &PendingChanges,
    muxer: &str,
    temp_path: &str
) -> Vec<String> {
    let mut builder = FfmpegBuilder::new();

    let main_input_idx = builder.add_input(file_path);

    // --- VIDEO ---
    builder.map(main_input_idx, "v:0?");

    if let Some(v) = &changes.video {
        let codec = v.codecName.as_deref().unwrap_or("copy");
        builder.codec("v:0", codec);

        if codec != "copy" {
            if let (Some(w), Some(h)) = (v.width, v.height) {
                builder.filter("v:0", &format!("scale={}:{}", w, h));
            }
            if let Some(ar) = &v.aspectRatio {
                builder.arg("-aspect", ar);
            }
            if let Some(pf) = &v.pixelFormat {
                builder.arg("-pix_fmt", pf);
            }
            if let Some(pr) = &v.profile {
                builder.arg("-profile:v", pr);
            }
        }
    } else {
        builder.codec("v:0", "copy");
    }

    // --- AUDIO ---
    if let Some(audios) = &changes.audio {
        let mut out_a_idx = 0;
        let mut original_a_idx = 0;

        for aud in audios {
            if aud.isDeleted {
                if !aud.isImported {
                    original_a_idx += 1;
                }
                continue;
            }

            if aud.isImported {
                let f_idx = builder.add_input(aud.path.as_ref().unwrap());
                builder.map(f_idx, "a:0");
            } else {
                builder.map(main_input_idx, &format!("a:{}", original_a_idx));
                original_a_idx += 1;
            }

            let acodec = aud.codecName.as_deref().unwrap_or("copy");
            let out_spec = format!("a:{}", out_a_idx);
            builder.codec(&out_spec, acodec);

            if acodec != "copy" {
                if let Some(br) = &aud.bitRate {
                    builder.arg(&format!("-b:{}", out_spec), br);
                }
                if let Some(sr) = &aud.sampleRate {
                    builder.arg(&format!("-ar:{}", out_spec), sr);
                }

                if
                    let (Some(new_ch), Some(cmap), Some(cvol)) = (
                        aud.newChannels,
                        &aud.channelMap,
                        &aud.channelVolumes,
                    )
                {
                    let new_ch_i = new_ch as i64;
                    let mut pan_parts = vec![format!("{}c", new_ch_i)];

                    for i in 0..new_ch_i {
                        let key = i.to_string();
                        if
                            let Some(val) = cmap
                                .get(&key)
                                .and_then(|v| if v.is_null() { None } else { Some(v) })
                        {
                            if
                                let Some(in_ch_f) = val
                                    .as_f64()
                                    .or_else(|| val.as_str().and_then(|s| s.parse().ok()))
                            {
                                let vol_db = cvol
                                    .get(&key)
                                    .and_then(|v| {
                                        v.as_f64().or_else(||
                                            v.as_str().and_then(|s| s.parse().ok())
                                        )
                                    })
                                    .unwrap_or(0.0);

                                let multiplier = (10_f64).powf(vol_db / 20.0);
                                pan_parts.push(
                                    format!("c{}={:.4}*c{}", i, multiplier, in_ch_f as i64)
                                );
                            }
                        }
                    }
                    builder.filter(&out_spec, &format!("pan={}", pan_parts.join("|")));
                } else if let Some(ac) = aud.newChannels {
                    builder.arg(&format!("-ac:{}", out_spec), &ac.to_string());
                }
            }

            let title = if aud.title.trim().is_empty() { "" } else { &aud.title };
            builder.metadata(&out_spec, "title", title);
            builder.metadata(&out_spec, "language", &aud.language);

            let mut flags = Vec::new();
            if aud.default {
                flags.push("default");
            }
            if aud.forced {
                flags.push("forced");
            }

            let disp_str = if flags.is_empty() { "0".to_string() } else { flags.join("+") };
            builder.disposition(&out_spec, &disp_str);

            out_a_idx += 1;
        }
    } else {
        builder.map(main_input_idx, "a?");
        builder.codec("a", "copy");
    }

    // --- SUBTITLE ---
    if let Some(subs) = &changes.subtitle {
        let mut original_idx = 0;
        let mut output_idx = 0;

        for sub in subs {
            if sub.isDeleted {
                if !sub.isImported {
                    original_idx += 1;
                }
                continue;
            }

            if sub.isImported {
                let f_idx = builder.add_input(sub.path.as_ref().unwrap());
                builder.map(f_idx, "s:0");
            } else {
                builder.map(main_input_idx, &format!("s:{}", original_idx));
                original_idx += 1;
            }

            let out_spec = format!("s:{}", output_idx);
            builder.codec(&out_spec, "copy");

            let title = if sub.title.trim().is_empty() { "" } else { &sub.title };
            builder.metadata(&out_spec, "title", title);
            builder.metadata(&out_spec, "language", &sub.language);

            let mut flags = Vec::new();
            if sub.default {
                flags.push("default");
            }
            if sub.forced {
                flags.push("forced");
            }

            let disp_str = if flags.is_empty() { "0".to_string() } else { flags.join("+") };
            builder.disposition(&out_spec, &disp_str);

            output_idx += 1;
        }
    } else {
        builder.map(main_input_idx, "s?");
        builder.codec("s", "copy");
    }

    builder.build(muxer, temp_path)
}

#[tauri::command]
pub async fn save_media_props(
    app: AppHandle,
    file_path: String,
    changes: PendingChanges,
    save_as: bool
) -> Result<String, String> {
    let info = match get_media_info(&app, &file_path).await {
        Ok(i) => i,
        Err(e) => {
            let msg = format!("Failed to read media info: {}", e);
            let _ = app.emit("backend-error", &msg);
            return Err(msg);
        }
    };

    let mut target_path = file_path.clone();

    if save_as {
        let path_ref = Path::new(&file_path);
        let ext = path_ref
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("mkv");
        let stem = path_ref
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("output");

        if
            let Some(p) = app
                .dialog()
                .file()
                .add_filter("Format", &[ext])
                .set_title("Save As")
                .set_file_name(&format!("{}_edited", stem))
                .blocking_save_file()
        {
            target_path = p.to_string();
        } else {
            return Err("Cancelled".into());
        }
    }

    let temp_path = format!("{}.tmp", target_path);
    let args = build_ffmpeg_args(&file_path, &changes, &info.muxer, &temp_path);

    let _ = app.emit("ffmpeg-log", format!("> ffmpeg {}", args.join(" ")));

    let (mut rx, _child) = app
        .shell()
        .sidecar("ffmpeg")
        .map_err(|e| {
            let msg = format!("Failed to find FFmpeg sidecar: {}", e);
            let _ = app.emit("backend-error", &msg);
            msg
        })?
        .args(args)
        .spawn()
        .map_err(|e| {
            let msg = format!("Failed to start FFmpeg process: {}", e);
            let _ = app.emit("backend-error", &msg);
            msg
        })?;

    let mut success = false;
    let mut error_log = String::new();

    while let Some(event) = rx.recv().await {
        match event {
            CommandEvent::Stderr(line) => {
                let out = String::from_utf8_lossy(&line);
                error_log.push_str(&out);
                error_log.push('\n');

                let _ = app.emit("ffmpeg-log", out.to_string());
            }
            CommandEvent::Terminated(p) => {
                success = p.code == Some(0);
            }
            _ => {}
        }
    }

    if success {
        thread::sleep(Duration::from_millis(150));

        if save_as {
            fs
                ::rename(&temp_path, &target_path)
                .or_else(|_| {
                    fs::copy(&temp_path, &target_path)?;
                    fs::remove_file(&temp_path)
                })
                .map_err(|e| {
                    let msg = format!("Failed to save new file: {}", e);
                    let _ = app.emit("backend-error", &msg);
                    msg
                })?;
        } else {
            let bak_path = format!("{}.bak", file_path);

            fs
                ::rename(&file_path, &bak_path)
                .or_else(|_| {
                    fs::copy(&file_path, &bak_path)?;
                    fs::remove_file(&file_path)
                })
                .map_err(|e| {
                    let msg = format!("Failed to create backup: {}", e);
                    let _ = app.emit("backend-error", &msg);
                    msg
                })?;

            match
                fs::rename(&temp_path, &target_path).or_else(|_| {
                    fs::copy(&temp_path, &target_path)?;
                    fs::remove_file(&temp_path)
                })
            {
                Ok(_) => {
                    let _ = fs::remove_file(&bak_path);
                }
                Err(e) => {
                    let _ = fs
                        ::rename(&bak_path, &file_path)
                        .or_else(|_| {
                            fs::copy(&bak_path, &file_path).and_then(|_| fs::remove_file(&bak_path))
                        });

                    let msg =
                        format!("Failed to overwrite original file. Backup restored. Error: {}", e);
                    let _ = app.emit("backend-error", &msg);
                    return Err(msg);
                }
            }
        }

        let success_msg = "File saved successfully";
        let _ = app.emit("backend-success", success_msg);
        Ok(success_msg.into())
    } else {
        let _ = fs::remove_file(&temp_path);
        let err_msg = format!("FFmpeg failed:\n{}", error_log);
        let _ = app.emit("backend-error", &err_msg);
        Err(err_msg)
    }
}
