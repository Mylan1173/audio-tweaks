use serde::Serialize;
use std::{ fs::{ self }, path::PathBuf };
use tauri::AppHandle;
use tauri_plugin_dialog::DialogExt;
#[derive(Serialize)]
pub enum DataType {
    File,
    Folder,
}
#[derive(Serialize)]
pub struct Data {
    children: Option<Vec<Data>>,
    data_type: DataType,
    data_name: String,
    data_path: PathBuf,
}

fn validate_file(path: PathBuf) -> bool {
    let allowlist = ["mp4", "m4p", "mkv", "avi"];

    match path.extension().and_then(|ext| ext.to_str()) {
        Some(ext) => allowlist.contains(&ext.to_lowercase().as_str()),
        None => false, // No extension found
    }
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
                data_type: DataType::Folder,
                data_name: content
                    .file_name()
                    .map(|n| n.to_string_lossy().into_owned())
                    .unwrap_or_else(|| path.to_string_lossy().into_owned()),
                children: dir_contents.unwrap().children,
                data_path: content,
            });
        } else {
            if validate_file(content.clone()) {
                contents.push(Data {
                    data_type: DataType::File,
                    data_name: content
                        .file_name()
                        .map(|n| n.to_string_lossy().into_owned())
                        .unwrap_or_else(|| path.to_string_lossy().into_owned()),
                    children: None,
                    data_path: content,
                });
            }
        }
    }
    Ok(Data {
        data_type: DataType::Folder,
        data_name: path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| path.to_string_lossy().into_owned()),
        children: Some(contents),
        data_path: path,
    })
}

#[tauri::command]
pub fn select_media(
    app: AppHandle,
    is_file: bool,
    refresh_path: Option<String>
) -> Result<Data, String> {
    let path_buf: PathBuf;
    if let Some(refresh_path) = refresh_path {
        path_buf = PathBuf::from(refresh_path);
    } else {
        let path: Option<tauri_plugin_fs::FilePath>;
        if is_file {
            path = app.dialog().file().set_title("Select a file").blocking_pick_file();
        } else {
            path = app.dialog().file().set_title("Select a folder").blocking_pick_folder();
        }
        path_buf = match path {
            Some(p) => p.as_path().unwrap().to_owned(),
            None => {
                return Err(String::from("Failed to get FilePath!"));
            }
        };
    }

    if is_file {
        Ok(Data {
            data_type: DataType::File,
            data_name: path_buf
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_else(|| path_buf.to_string_lossy().into_owned()),
            children: None,
            data_path: path_buf,
        })
    } else {
        match get_folder_contents(path_buf) {
            Ok(p) => Ok(p),
            Err(e) => Err(e),
        }
    }
}
