use std::fs::File;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::Path;

use tauri::command;

use crate::read_folder;
use crate::Folder;
use crate::STATIC_DIR;

#[command]
pub fn open_folder(dir: String) -> Vec<Folder> {
    unsafe {
        STATIC_DIR = Box::leak(dir.into_boxed_str());
        let path = Path::new(STATIC_DIR);
        read_folder(path, 0)
    }
}

#[command]
pub fn read_text_file(dir: String) -> String {
    let mut file = File::open(dir).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    buf
}

#[command]
pub fn write_text_file(dir: String, text: String) {
    let mut file = OpenOptions::new().write(true).truncate(true).open(dir).unwrap();
    file.write_all(text.as_bytes()).expect("write failed");
}
