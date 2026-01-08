use std::{ fs, path::PathBuf };
use tauri::AppHandle;
use tauri_plugin_dialog::DialogExt;
use serde::Serialize;
#[derive(Serialize)]
enum FileType {
    File,
    Folder,
}
#[derive(Serialize)]
struct Data {
    d_type: FileType,
    name: String,
    children: Option<Vec<Data>>,
    pb: PathBuf,
}

fn get_folder_contents(path: PathBuf) -> Result<Data, String> {
    let r_dir: fs::ReadDir = match fs::read_dir(&path) {
        Ok(r) => r,
        Err(_) => {
            return Err(String::from("Failed to read dir!"));
        }
    };

    let contents_pathbuf: Vec<PathBuf> = r_dir
        .filter_map(|content| Some(content.ok()?.path()))
        .collect();

    let mut contents: Vec<Data> = Vec::new();

    for content in contents_pathbuf {
        if content.is_dir() {
            let dir_contents = get_folder_contents(content.clone());

            contents.push(Data {
                d_type: FileType::Folder,
                name: content
                    .file_name()
                    .map(|n| n.to_string_lossy().into_owned())
                    .unwrap_or_else(|| path.to_string_lossy().into_owned()),
                children: dir_contents.unwrap().children,
                pb: content,
            });
        } else {
            contents.push(Data {
                d_type: FileType::File,
                name: content
                    .file_name()
                    .map(|n| n.to_string_lossy().into_owned())
                    .unwrap_or_else(|| path.to_string_lossy().into_owned()),
                children: None,
                pb: content,
            });
        }
    }
    Ok(Data {
        d_type: FileType::Folder,
        name: path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| path.to_string_lossy().into_owned()),
        children: Some(contents),
        pb: path,
    })
}

#[tauri::command]
fn select_media(app: AppHandle, is_file: bool) -> Result<Data, String> {
    let path: Option<tauri_plugin_fs::FilePath>;
    if is_file {
        path = app.dialog().file().set_title("Select a file").blocking_pick_file();
    } else {
        path = app.dialog().file().set_title("Select a folder").blocking_pick_folder();
    }
    let path_buf: PathBuf = match path {
        Some(p) => p.as_path().unwrap().to_owned(),
        None => {
            return Err(String::from("Failed to get FilePath!"));
        }
    };
    if is_file {
        Ok(Data {
            d_type: FileType::File,
            name: path_buf
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_else(|| path_buf.to_string_lossy().into_owned()),
            children: None,
            pb: path_buf,
        })
    } else {
        match get_folder_contents(path_buf) {
            Ok(p) => Ok(p),
            Err(e) => Err(e),
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder
        ::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![select_media])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
