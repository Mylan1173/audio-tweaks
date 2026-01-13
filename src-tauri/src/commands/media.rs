use tauri_plugin_shell::ShellExt;
use serde_json::Value;

#[tauri::command]
pub async fn get_media_streams(app: tauri::AppHandle, path: String) -> Result<Value, String> {
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
