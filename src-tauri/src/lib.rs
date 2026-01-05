use std::{ fs, path::Path, path::PathBuf, thread };
use tauri_plugin_dialog::DialogExt;
use tauri::AppHandle;

#[tauri::command]
async fn list_audio_files(app: &str) -> Result<Vec<String>, String> {
    let audio_path: &Path = Path::new(app);

    let entries = match fs::read_dir(&audio_path) {
        Err(why) => {
            return Err(why.to_string());
        }
        Ok(f) => f,
    };

    let file_list: Vec<String> = entries
        .filter_map(|entry| Some(entry.ok()?.path().to_string_lossy().into_owned()))
        .collect();

    Ok(file_list)
}

#[tauri::command]
fn select_media(app: AppHandle) -> Result<String, String> {
    let file_path = app
        .dialog()
        .file()
        .set_title("Select a folder to be added")
        .blocking_pick_folder();
    let path: PathBuf = match file_path {
        Some(p) => p.as_path().unwrap().to_owned(),
        None => {
            return Ok(String::from("Nothing!"));
        }
    };
    Ok(path.to_string_lossy().into_owned())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder
        ::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![list_audio_files, select_media])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
