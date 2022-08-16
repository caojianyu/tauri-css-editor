#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{fs, path::Path, thread, time};

use uuid::Uuid;

use tauri::Manager;

use commands::{open_folder, read_text_file, write_text_file};
mod commands;

#[derive(Debug, Clone, serde::Serialize)]
pub struct Folder {
    name: String,
    id: String,
    level: u32,
    is_dir: bool,
    file_path: String,
    extension: String,
    chidren: Vec<Folder>,
}

static mut STATIC_DIR: &str = "";
// static mut STATIC_CUREENT_FILE_PATH: &str = "";

fn read_folder(path: &Path, mut level: u32) -> Vec<Folder> {
    level += 1;
    let paths = fs::read_dir(path).unwrap();
    let mut folder_arr: Vec<Folder> = Vec::new();
    for p in paths {
        let uuid = Uuid::new_v4();
        let id = uuid.to_string().replace("-", "");

        let pb = p.unwrap().path();
        let name = pb
            .file_name()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap();

        let is_dir = pb.is_dir();

        let mut folder = Folder {
            name,
            id,
            level,
            is_dir,
            file_path: pb.as_path().to_str().unwrap().to_string(),
            extension: String::new(),
            chidren: Vec::new(),
        };
        if is_dir {
            folder.chidren = read_folder(pb.as_path(), level);
        }

        match pb.extension() {
            Some(val) => folder.extension = val.to_os_string().into_string().unwrap(),
            None => {}
        }

        folder_arr.push(folder);
    }

    folder_arr
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();
            thread::spawn(move || loop {
                unsafe {
                    if !STATIC_DIR.is_empty() {
                        let path = Path::new(STATIC_DIR);
                        main_window
                            .emit("open_folder", read_folder(path, 0))
                            .unwrap();
                        thread::sleep(time::Duration::from_millis(10000));
                    }
                }
            });

            // thread::spawn(move || loop {
            //     unsafe {
            //         if !STATIC_CUREENT_FILE_PATH.is_empty() {
            //             let mut file = fs::File::open(STATIC_CUREENT_FILE_PATH).unwrap();
            //             let mut buf = String::new();
            //             file.read_to_string(&mut buf).unwrap();

            //             win.emit("read_file", buf).unwrap();
            //             thread::sleep(time::Duration::from_millis(500));
            //         }
            //     }
            // });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            open_folder,
            read_text_file,
            write_text_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
