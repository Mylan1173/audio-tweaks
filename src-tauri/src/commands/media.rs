use tauri::AppHandle;
use tauri_plugin_shell::ShellExt;
use serde_json::Value;
use serde::{ Deserialize, Serialize };

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
}

#[tauri::command]
pub async fn save_media_props(
    app: AppHandle,
    file_path: String,
    changes: PendingChanges
) -> Result<(), String> {
    let mut ffmpeg_args = vec![
        "-i".to_string(),
        file_path.clone(),
        "-map".to_string(),
        "0".to_string(),
        "-c".to_string(),
        "copy".to_string()
    ];

    if let Some(subs) = changes.subs {
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

    let output_path = format!("{}_tweaked.mkv", file_path);
    ffmpeg_args.push(output_path);

    let cmd = app.shell().sidecar("ffmpeg").unwrap().args(ffmpeg_args);
    let _output = cmd.output().await.map_err(|e| e.to_string())?;

    return Ok(());
}
