mod commands;

use commands::{ explorer, media };

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder
        ::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(
            tauri::generate_handler![
                explorer::select_media,
                media::get_media_streams,
                media::save_media_props
            ]
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
