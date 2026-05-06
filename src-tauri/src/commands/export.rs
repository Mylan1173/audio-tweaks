use std::path::Path;
use std::fs;
use tauri::{ AppHandle, Emitter };
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_shell::{ ShellExt, process::CommandEvent };

#[tauri::command]
pub async fn export_stream(
    app: AppHandle,
    input_path: String,
    stream_type: String,
    stream_index: usize
) -> Result<(), String> {
    let default_ext = match stream_type.as_str() {
        "video" => "mkv",
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

    let mut dialog_builder = app
        .dialog()
        .file()
        .set_title(format!("Export {} Stream", stream_type));

    if stream_type == "audio" {
        dialog_builder = dialog_builder
            .add_filter("MP3 Audio", &["mp3"])
            .add_filter("WAV Audio (Uncompressed)", &["wav"])
            .add_filter("FLAC Audio (Lossless)", &["flac"])
            .add_filter("AAC Audio", &["aac"])
            .add_filter("M4A Audio", &["m4a"])
            .add_filter("Ogg Vorbis", &["ogg"])
            .add_filter("Matroska Audio (Zero-Loss Copy)", &["mka"]);
    } else if stream_type == "video" {
        dialog_builder = dialog_builder
            .add_filter("Matroska Video", &["mkv"])
            .add_filter("MP4 Video", &["mp4"])
            .add_filter("AVI Video", &["avi"])
            .add_filter("QuickTime Video", &["mov"])
            .add_filter("WebM Video", &["webm"]);
    } else if stream_type == "subtitle" {
        dialog_builder = dialog_builder
            .add_filter("SubRip Subtitle", &["srt"])
            .add_filter("Advanced SubStation Alpha", &["ass"])
            .add_filter("WebVTT", &["vtt"]);
    }

    let target_path = dialog_builder
        .set_file_name(
            format!(
                "{}_{}_track-{}_index-{}.{}",
                file_stem,
                file_extension,
                stream_type,
                stream_index,
                default_ext
            )
        )
        .blocking_save_file();

    let output_path: String = match target_path {
        Some(path) => path.to_string(),
        None => {
            return Ok(());
        }
    };

    let output_path_buf = Path::new(&output_path);
    let chosen_ext = output_path_buf
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or(default_ext)
        .to_lowercase();

    let stream_specifier = match stream_type.as_str() {
        "video" => format!("0:v:{}", stream_index),
        "audio" => format!("0:a:{}", stream_index),
        "subtitle" => format!("0:s:{}", stream_index),
        _ => format!("0:{}", stream_index),
    };

    let mut args = vec!["-i".to_string(), input_path, "-map".to_string(), stream_specifier];

    if stream_type == "audio" {
        match chosen_ext.as_str() {
            "mp3" => {
                args.push("-c:a".to_string());
                args.push("libmp3lame".to_string());
                args.push("-q:a".to_string());
                args.push("2".to_string());
            }
            "wav" => {
                args.push("-c:a".to_string());
                args.push("pcm_s16le".to_string());
            }
            "flac" => {
                args.push("-c:a".to_string());
                args.push("flac".to_string());
            }
            "aac" | "m4a" => {
                args.push("-c:a".to_string());
                args.push("aac".to_string());
                args.push("-b:a".to_string());
                args.push("192k".to_string());
            }
            "ogg" => {
                args.push("-c:a".to_string());
                args.push("libvorbis".to_string());
                args.push("-q:a".to_string());
                args.push("4".to_string());
            }
            _ => {
                args.push("-c:a".to_string());
                args.push("copy".to_string());
            }
        }
    } else {
        args.push("-c".to_string());
        args.push("copy".to_string());
    }

    args.push("-y".to_string());
    args.push(output_path.clone());

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
            let msg = format!("Failed to spawn FFmpeg: {}", e);
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
        let _ = app.emit(
            "backend-success",
            format!("{} stream exported successfully", stream_type)
        );
        Ok(())
    } else {
        let _ = fs::remove_file(&output_path);

        let err_msg = format!("FFmpeg export failed:\n{}", error_log);
        let _ = app.emit("backend-error", &err_msg);
        Err(err_msg)
    }
}
