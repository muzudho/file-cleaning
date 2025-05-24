// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

/*
use sha2::{Digest, Sha256};
use std::fs;
use std::io;
*/

/*
#[tauri::command]
fn scan_files(path: String) -> Vec<String> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(file) = entry {
                let path = file.path();
                if path.is_file() {
                    files.push(path.to_string_lossy().to_string());
                }
            }
        }
    }
    files
}

#[tauri::command]
fn get_file_hash(path: String) -> String {
    let mut file = fs::File::open(&path).unwrap();
    let mut hasher = Sha256::new();
    io::copy(&mut file, &mut hasher).unwrap();
    format!("{:x", hasher.finalize())
}

#[tauri::command]
fn delete_file(src: String, dest: String) -> bool{
    fs::rename(src, dest).is_ok()
}
*/

fn main() {
    //*
    file_cleaning_lib::run()
    // */
    /*
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    // */
    /*
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            scan_files,
            get_file_hash,
            delete_file,
            move_file
        ])
        .run(tauri::generate_context!())
        .expect("Error running Tauri app");
    // */
}
