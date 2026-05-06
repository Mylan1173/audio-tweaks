use tauri::{ AppHandle, Emitter };
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

            match fs::write(&path_str, profile_data) {
                Ok(_) => {
                    let _ = app.emit("backend-success", "Profile exported successfully");
                    Ok(path_str)
                }
                Err(e) => {
                    let msg = format!("Failed to write profile: {}", e);
                    let _ = app.emit("backend-error", &msg);
                    Err(msg)
                }
            }
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
            let path_str = path.to_string();

            match fs::read_to_string(&path_str) {
                Ok(content) => {
                    let _ = app.emit("backend-success", "Profile imported successfully");
                    Ok(Some(content))
                }
                Err(e) => {
                    let msg = format!("Failed to read profile: {}", e);
                    let _ = app.emit("backend-error", &msg);
                    Err(msg)
                }
            }
        }
        None => Ok(None),
    }
}
