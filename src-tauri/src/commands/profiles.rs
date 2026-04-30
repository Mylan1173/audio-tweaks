use tauri::AppHandle;
use tauri_plugin_dialog::DialogExt;
use std::fs;

#[tauri::command]
pub async fn export_profile(app: AppHandle, profile_data: String) -> Result<String, String> {
    let file_path = app
        .dialog()
        .file()
        .add_filter("JSON Profile", &["json"])
        .set_file_name("batch_profile.json")
        .set_title("Export Batch Profile")
        .blocking_save_file();

    match file_path {
        Some(path) => {
            let path_str = path.to_string();
            fs
                ::write(&path_str, profile_data)
                .map_err(|e| format!("Failed to write profile: {}", e))?;
            Ok(path_str)
        }
        None => Ok("Cancelled".into()),
    }
}

#[tauri::command]
pub async fn import_profile(app: AppHandle) -> Result<Option<String>, String> {
    let file_path = app
        .dialog()
        .file()
        .add_filter("JSON Profile", &["json"])
        .set_title("Import Batch Profile")
        .blocking_pick_file();

    match file_path {
        Some(path) => {
            let content = fs
                ::read_to_string(path.as_path().unwrap())
                .map_err(|e| format!("Failed to read profile: {}", e))?;
            Ok(Some(content))
        }
        None => Ok(None),
    }
}
