use std::{ fs, path::Path };

#[tauri::command]
async fn list_audio_files(app: &str) -> Result<Vec<String>, String> {
    // 1. Resolve the $AUDIO directory path
    let audio_path: &Path = Path::new(app);

    // 2. Read the directory
    let entries = match fs::read_dir(&audio_path) {
        Err(why) => {
            return Err(why.to_string());
        }
        Ok(f) => f,
    };

    // 3. Process the iterator and collect into a Vec
    let file_list: Vec<String> = entries
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            // Only include files (skip directories) and check for audio extensions
            if path.is_file() {
                return Some(path.to_string_lossy().into_owned());
            }
            None
        })
        .collect(); // <--- THIS is what fixes your "Map" error

    Ok(file_list)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder
        ::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![list_audio_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
