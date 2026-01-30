use tauri::{ AppHandle, Emitter };
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_shell::{ ShellExt, process::CommandEvent };
use std::{ fs, path::Path };
use serde_json::Value;
use serde::{ Deserialize, Serialize };
use std::{ thread, time::Duration };

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
            "stream=index,codec_name,codec_type,bit_rate,channels,sample_rate:stream_tags=language,title:disposition=default,forced",
            "-of",
            "json",
            &path,
        ])
        .output().await
        .map_err(|e| e.to_string())?;

    let json: Value = serde_json
        ::from_slice(&output.stdout)
        .map_err(|e| format!("Failed to parse ffprobe output: {}", e))?;

    Ok(json)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PendingChanges {
    pub subs: Option<SubtitleChanges>,
    //TBD
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SubtitleChanges {
    pub default: Option<usize>,
    pub forced: Vec<usize>,
    pub delete: Vec<usize>,
}

#[tauri::command]
pub async fn save_media_props(
    app: AppHandle,
    file_path: String,
    changes: PendingChanges,
    save_as: bool
) -> Result<(), String> {
    // 1. Get duration and muxer
    let probe_output = app
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
            &file_path,
        ])
        .output().await
        .map_err(|e| e.to_string())?;

    let probe_res = String::from_utf8_lossy(&probe_output.stdout);
    let mut muxer = "matroska";
    let mut total_duration_secs: f64 = 0.0;

    for line in probe_res.lines() {
        if line.starts_with("format_name=") {
            muxer = line
                .split('=')
                .last()
                .unwrap_or("matroska")
                .split(',')
                .next()
                .unwrap_or("matroska");
        }
        if line.starts_with("duration=") {
            total_duration_secs = line
                .split('=')
                .last()
                .unwrap_or("0")
                .parse::<f64>()
                .unwrap_or(0.0);
        }
    }
    let mut target_path = file_path.clone();
    if save_as {
        let path = Path::new(&file_path);
        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("mkv");

        let file_stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("output");

        let file_path = app
            .dialog()
            .file()
            .add_filter("Original Format", &[extension])
            .set_title("Save Media As")
            .set_file_name(&format!("{}_edited", file_stem))
            .blocking_save_file();
        if let Some(t_path) = file_path {
            target_path = t_path.to_string();
        } else {
            return Err("Save cancelled".to_string());
        }
    }

    let total_micros = (total_duration_secs * 1_000_000.0) as i64;
    let temp_path = format!("{}.tmp", file_path);

    // 2. Build FFmpeg Args
    let mut ffmpeg_args = vec![
        "-y".to_string(),
        "-hide_banner".to_string(),
        "-hwaccel".to_string(),
        "auto".to_string(),
        "-thread_queue_size".to_string(),
        "1024".to_string(),
        "-i".to_string(),
        file_path.clone(),
        "-map".to_string(),
        "0".to_string(),
        "-c".to_string(),
        "copy".to_string(),
        "-map_metadata".to_string(),
        "0".to_string(),
        "-progress".to_string(),
        "pipe:1".to_string(),
        "-nostats".to_string()
    ];

    if let Some(subs) = changes.subs {
        if let Some(delete_indices) = Some(subs.delete) {
            for idx in delete_indices {
                ffmpeg_args.push("-map".to_string());
                ffmpeg_args.push(format!("-0:s:{}", idx));
            }
        }

        ffmpeg_args.push("-disposition:s".to_string());
        ffmpeg_args.push("0".to_string());

        if let Some(def_idx) = subs.default {
            ffmpeg_args.push(format!("-disposition:s:{}", def_idx));
            ffmpeg_args.push("default".to_string());
        }

        for idx in subs.forced.iter() {
            ffmpeg_args.push(format!("-disposition:s:{}", idx));
            ffmpeg_args.push("forced".to_string());
        }
    }

    ffmpeg_args.push("-f".to_string());
    ffmpeg_args.push(muxer.to_string());
    ffmpeg_args.push(temp_path.clone());

    // 3. Spawn and Calculate Percentage
    let (mut rx, _child) = app
        .shell()
        .sidecar("ffmpeg")
        .map_err(|e| e.to_string())?
        .args(ffmpeg_args)
        .spawn()
        .map_err(|e| e.to_string())?;

    let mut success = false;
    let mut last_percent = 0;

    while let Some(event) = rx.recv().await {
        match event {
            CommandEvent::Stdout(line) => {
                let out = String::from_utf8_lossy(&line);
                if out.contains("out_time_ms=") && total_micros > 0 {
                    let ms_str = out.split('=').last().unwrap_or("0").trim();
                    if let Ok(current_micros) = ms_str.parse::<i64>() {
                        // Math: (Current / Total) * 100
                        let percent = (((current_micros as f64) / (total_micros as f64)) *
                            100.0) as i32;

                        // Only emit if the percentage actually increased (prevents flooding)
                        if percent > last_percent && percent <= 100 {
                            last_percent = percent;
                            let _ = app.emit("ffmpeg-progress", percent);
                        }
                    }
                }
            }
            CommandEvent::Terminated(payload) => {
                success = payload.code == Some(0);
            }
            _ => {}
        }
    }

    // 4. File Swap Logic
    if success {
        let _ = app.emit("ffmpeg-progress", 100);
        thread::sleep(Duration::from_millis(150));

        if !save_as {
            fs::remove_file(&file_path).map_err(|e| e.to_string())?;
        }
        fs::rename(&temp_path, &target_path).map_err(|e| e.to_string())?;
        Ok(())
    } else {
        let _ = fs::remove_file(&temp_path);
        Err("FFmpeg failed to process file".to_string())
    }
}
