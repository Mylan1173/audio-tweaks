use tauri::{ AppHandle, Emitter, Manager };
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_shell::{ ShellExt, process::CommandEvent };
use std::{ collections::HashMap, fs, path::Path, thread, time::Duration };
use serde::{ Deserialize, Serialize };
use serde_json::Value;
use crate::export::get_duration;

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
        .map_err(|e| e.to_string())?
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
        .map_err(|e| e.to_string())?;

    serde_json::from_slice(&output.stdout).map_err(|e| format!("Failed to parse ffprobe: {}", e))
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
        .map_err(|e| e.to_string())?;

    if !cache_dir.exists() {
        std::fs::create_dir_all(&cache_dir).map_err(|e| e.to_string())?;
    }

    let output_path = cache_dir.join("temp_visualizer.wav");
    let output_path_str = output_path.to_str().ok_or("Invalid cache path")?.to_string();

    let total_duration = get_duration(&app, &input_path).await?;
    let total_micros = (total_duration * 1_000_000.0) as i64;

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

    let (mut rx, _child) = app
        .shell()
        .sidecar("ffmpeg")
        .map_err(|e| e.to_string())?
        .args(args)
        .spawn()
        .map_err(|e| e.to_string())?;

    let mut last_percent = 0;

    while let Some(event) = rx.recv().await {
        if let CommandEvent::Stdout(line) = event {
            let out = String::from_utf8_lossy(&line);
            if out.contains("out_time_ms=") && total_micros > 0 {
                let ms_str = out.split('=').last().unwrap_or("0").trim();
                if let Ok(current_micros) = ms_str.parse::<i64>() {
                    let percent = (((current_micros as f64) / (total_micros as f64)) *
                        100.0) as i32;
                    if percent > last_percent && percent <= 100 {
                        last_percent = percent;
                        let _ = app.emit("ffmpeg-progress", percent);
                    }
                }
            }
        }
    }

    app.emit("ffmpeg-progress", 100).ok();

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
    let mut args = vec![
        "-y".into(),
        "-hide_banner".into(),
        "-hwaccel".into(),
        "auto".into(),
        "-i".into(),
        file_path.into()
    ];

    let mut imported_files = Vec::new();
    if let Some(subs) = &changes.subtitle {
        for sub in subs {
            if sub.isImported {
                if let Some(p) = &sub.path {
                    if !imported_files.contains(p) {
                        args.push("-i".into());
                        args.push(p.clone());
                        imported_files.push(p.clone());
                    }
                }
            }
        }
    }
    if let Some(audios) = &changes.audio {
        for aud in audios {
            if aud.isImported {
                if let Some(p) = &aud.path {
                    if !imported_files.contains(p) {
                        args.push("-i".into());
                        args.push(p.clone());
                        imported_files.push(p.clone());
                    }
                }
            }
        }
    }

    // --- VIDEO ---
    args.push("-map".into());
    args.push("0:v:0?".into());

    if let Some(v) = &changes.video {
        let codec = v.codecName.clone().unwrap_or_else(|| "copy".into());
        args.push("-c:v:0".into());
        args.push(codec.clone());

        if codec != "copy" {
            if let (Some(w), Some(h)) = (v.width, v.height) {
                args.push("-vf".into());
                args.push(format!("scale={}:{}", w, h));
            }
            if let Some(ar) = &v.aspectRatio {
                args.push("-aspect".into());
                args.push(ar.clone());
            }
            if let Some(pf) = &v.pixelFormat {
                args.push("-pix_fmt".into());
                args.push(pf.clone());
            }
            if let Some(pr) = &v.profile {
                args.push("-profile:v".into());
                args.push(pr.clone());
            }
        }
    } else {
        args.push("-c:v:0".into());
        args.push("copy".into());
    }

    // --- AUDIO ---
    if let Some(audios) = &changes.audio {
        let mut out_a_idx = 0;
        let mut original_a_idx = 0;

        for aud in audios {
            if aud.isImported {
                let f_idx =
                    imported_files
                        .iter()
                        .position(|r| r == aud.path.as_ref().unwrap())
                        .unwrap() + 1;
                args.push("-map".into());
                args.push(format!("{}:a:0", f_idx));
            } else {
                if !aud.isDeleted {
                    args.push("-map".into());
                    args.push(format!("0:a:{}", original_a_idx));
                }
                original_a_idx += 1;
            }

            if aud.isDeleted {
                continue;
            }

            let acodec = aud.codecName.clone().unwrap_or_else(|| "copy".into());
            args.push(format!("-c:a:{}", out_a_idx));
            args.push(acodec.clone());

            if acodec != "copy" {
                if let Some(br) = &aud.bitRate {
                    args.push(format!("-b:a:{}", out_a_idx));
                    args.push(br.clone());
                }
                if let Some(sr) = &aud.sampleRate {
                    args.push(format!("-ar:a:{}", out_a_idx));
                    args.push(sr.clone());
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
                        if let Some(val) = cmap.get(&key) {
                            if !val.is_null() {
                                let in_ch = val
                                    .as_f64()
                                    .or_else(|| val.as_str().and_then(|s| s.parse().ok()));

                                if let Some(in_ch_f) = in_ch {
                                    let vol_val = cvol.get(&key).unwrap_or(&Value::Null);
                                    let vol_db = vol_val
                                        .as_f64()
                                        .or_else(|| vol_val.as_str().and_then(|s| s.parse().ok()))
                                        .unwrap_or(0.0);

                                    let multiplier = (10_f64).powf(vol_db / 20.0);
                                    pan_parts.push(
                                        format!("c{}={:.4}*c{}", i, multiplier, in_ch_f as i64)
                                    );
                                }
                            }
                        }
                    }

                    args.push(format!("-filter:a:{}", out_a_idx));
                    args.push(format!("pan={}", pan_parts.join("|")));
                } else if let Some(ac) = aud.newChannels {
                    args.push(format!("-ac:a:{}", out_a_idx));
                    args.push(ac.to_string());
                }
            }

            args.push(format!("-metadata:s:a:{}", out_a_idx));
            args.push(
                format!("title={}", if aud.title.trim().is_empty() { "" } else { &aud.title })
            );
            args.push(format!("-metadata:s:a:{}", out_a_idx));
            args.push(format!("language={}", aud.language));

            let mut a_flags = Vec::new();
            if aud.default {
                a_flags.push("default");
            }
            if aud.forced {
                a_flags.push("forced");
            }

            args.push(format!("-disposition:a:{}", out_a_idx));
            args.push(if a_flags.is_empty() { "0".into() } else { a_flags.join("+") });

            out_a_idx += 1;
        }
    } else {
        args.push("-map".into());
        args.push("0:a?".into());
        args.push("-c:a".into());
        args.push("copy".into());
    }

    // --- SUBTITLE ---
    if let Some(subs) = &changes.subtitle {
        let mut original_idx = 0;
        let mut output_idx = 0;

        for sub in subs {
            let source = if sub.isImported {
                let f_idx =
                    imported_files
                        .iter()
                        .position(|r| r == sub.path.as_ref().unwrap())
                        .unwrap() + 1;
                format!("{}:s:0", f_idx)
            } else {
                let s = format!("0:s:{}", original_idx);
                original_idx += 1;
                s
            };

            if sub.isDeleted {
                continue;
            }

            args.push("-map".into());
            args.push(source);

            args.push(format!("-c:s:{}", output_idx));
            args.push("copy".into());

            args.push(format!("-metadata:s:s:{}", output_idx));
            args.push(
                format!("title={}", if sub.title.trim().is_empty() { "" } else { &sub.title })
            );

            args.push(format!("-metadata:s:s:{}", output_idx));
            args.push(format!("language={}", sub.language));

            let mut flags = Vec::new();
            if sub.default {
                flags.push("default");
            }
            if sub.forced {
                flags.push("forced");
            }

            args.push(format!("-disposition:s:{}", output_idx));
            args.push(if flags.is_empty() { "0".into() } else { flags.join("+") });

            output_idx += 1;
        }
    } else {
        args.push("-map".into());
        args.push("0:s?".into());
        args.push("-c:s".into());
        args.push("copy".into());
    }

    args.extend(
        vec![
            "-stats".into(),
            "-v".into(),
            "info".into(),
            "-f".into(),
            muxer.into(),
            temp_path.into()
        ]
    );
    args
}

#[tauri::command]
pub async fn save_media_props(
    app: AppHandle,
    file_path: String,
    changes: PendingChanges,
    save_as: bool
) -> Result<String, String> {
    let info = get_media_info(&app, &file_path).await?;
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

    let (mut rx, _child) = app
        .shell()
        .sidecar("ffmpeg")
        .map_err(|e| e.to_string())?
        .args(args)
        .spawn()
        .map_err(|e| format!("Failed to start FFmpeg process: {}", e))?;

    let mut last_percent = -1;
    let mut success = false;

    while let Some(event) = rx.recv().await {
        match event {
            CommandEvent::Stderr(line) => {
                let out = String::from_utf8_lossy(&line);
                if let Some(start) = out.find("time=") {
                    let ts = &out[start + 5..].split_whitespace().next().unwrap_or("");
                    if let Ok(secs) = parse_ffmpeg_time(ts) {
                        let percent = ((secs / info.duration) * 100.0) as i32;
                        let percent = percent.clamp(0, 100);
                        if percent > last_percent {
                            last_percent = percent;
                            let _ = app.emit("ffmpeg-progress", percent);
                        }
                    }
                }
            }
            CommandEvent::Terminated(p) => {
                success = p.code == Some(0);
            }
            _ => {}
        }
    }

    if success {
        let _ = app.emit("ffmpeg-progress", 100);
        thread::sleep(Duration::from_millis(150));
        if !save_as {
            fs::remove_file(&file_path).map_err(|e| e.to_string())?;
        }
        fs::rename(&temp_path, &target_path).map_err(|e| e.to_string())?;
        Ok("File saved successfully".into())
    } else {
        let _ = fs::remove_file(&temp_path);
        Err("FFmpeg failed".into())
    }
}

fn parse_ffmpeg_time(t: &str) -> Result<f64, String> {
    let p: Vec<&str> = t.split(':').collect();
    if p.len() != 3 {
        return Err("Invalid format".into());
    }
    let h: f64 = p[0].parse().unwrap_or(0.0);
    let m: f64 = p[1].parse().unwrap_or(0.0);
    let s: f64 = p[2].parse().unwrap_or(0.0);
    Ok(h * 3600.0 + m * 60.0 + s)
}
