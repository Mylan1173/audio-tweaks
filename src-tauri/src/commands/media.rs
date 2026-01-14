use tauri::AppHandle;
use tauri_plugin_shell::ShellExt;
use serde_json::Value;

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

async fn save_subtitle_prop(app: AppHandle, path: String) -> Result<(), String> {
    let arguments = [
        "-i",
        &path,
        "-map",
        "0",
        "-c",
        "copy",
        "-disposition:s",
        "0",
        &format!("-disposition:s:{}", 1),
        "default",
    ];

    let output = app
        .shell()
        .sidecar("ffmpeg")
        .map_err(|e| e.to_string())?
        .args(arguments)
        .output().await
        .map_err(|e| e.to_string())?;

    return Ok(());
}
