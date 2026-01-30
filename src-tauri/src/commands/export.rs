use std::path::Path;
use tauri::{ AppHandle, Emitter };
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_shell::{ ShellExt, process::CommandEvent };

#[tauri::command]
pub async fn export_stream(
    app: AppHandle,
    input_path: String,
    stream_type: String, // "video", "audio", "subtitle"
    stream_index: usize
) -> Result<(), String> {
    // 1. Determine extension based on type
    let extension = match stream_type.as_str() {
        "video" => "mp4",
        "audio" => "mp3",
        "subtitle" => "srt",
        _ => "mkv",
    };

    let path = Path::new(&input_path);
    let file_extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("mkv");

    let file_stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");

    // 2. Open Save Dialog
    let target_path = app
        .dialog()
        .file()
        .add_filter("Export Format", &[extension])
        .set_title(format!("Export {} Stream", stream_type))
        .set_file_name(
            format!(
                "{}_{}_track-{}_index-{}.{}",
                file_stem,
                file_extension,
                stream_type,
                stream_index,
                extension
            )
        )
        .blocking_save_file();

    let output_path: String = match target_path {
        Some(path) => path.to_string(),
        None => {
            return Ok(());
        }
    };

    // 3. Get Total Duration for progress calculation
    let total_duration = get_duration(&app, &input_path).await?;
    let total_micros = (total_duration * 1_000_000.0) as i64;

    // 4. Build FFmpeg args
    // We use "0:v:idx" or "0:s:idx" to target the specific stream type index
    let stream_specifier = match stream_type.as_str() {
        "video" => format!("0:v:{}", stream_index),
        "audio" => format!("0:a:{}", stream_index),
        "subtitle" => format!("0:s:{}", stream_index),
        _ => format!("0:{}", stream_index),
    };

    let args = vec![
        "-i".to_string(),
        input_path,
        "-map".to_string(),
        stream_specifier,
        "-c".to_string(),
        "copy".to_string(),
        "-progress".to_string(),
        "pipe:1".to_string(),
        "-nostats".to_string(),
        "-y".to_string(),
        output_path.clone()
    ];

    // 5. Spawn and monitor
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
    Ok(())
}

async fn get_duration(app: &AppHandle, path: &str) -> Result<f64, String> {
    let output = app
        .shell()
        .sidecar("ffprobe")
        .map_err(|e| e.to_string())?
        .args([
            "-v",
            "error",
            "-show_entries",
            "format=duration",
            "-of",
            "default=noprint_wrappers=1:nokey=1",
            path,
        ])
        .output().await
        .map_err(|e| e.to_string())?;

    String::from_utf8_lossy(&output.stdout)
        .trim()
        .parse::<f64>()
        .map_err(|e| e.to_string())
}
