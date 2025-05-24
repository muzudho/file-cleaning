// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//use tauri_plugin_fs::FsExt;
use sha2::{Digest, Sha256};
//use std::fs;
//use std::fs::File;
//use std::io;
//use std::io::Read;

//*
#[tauri::command]
fn scan_files_1(path: String) -> Vec<String> {
    let mut files = Vec::new();
    if let Ok(entries) = std::fs::read_dir(path) {
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
// */

/*
#[tauri::command]
fn scan_files_2(app: tauri::AppHandle, path: String) -> Result<Vec<String>, String> {
    let paths = app
        .fs()
        .read_dir(&path, None)
        .map_err(|e| e.to_string())?
        .into_iter()
        .filter(|e| e.path().is_file())
        .map(|e| e.path().to_string_lossy().to_string())
        .collect();
    Ok(paths)
}
// */

//*
#[tauri::command]
fn get_file_hash_1(path: String) -> String {
    let mut file = std::fs::File::open(&path).unwrap();
    let mut hasher = Sha256::new();
    std::io::copy(&mut file, &mut hasher).unwrap();
    format!("{:x}", hasher.finalize())
}
// */

/*
#[tauri::command]
fn get_file_hash_2(app: tauri::AppHandle, path: String) -> Result<String, String> {
    let mut file = File::open(&path).map_err(|e| e.to_string())?;
    let mut hasher = Sha256::new();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).map_err(|e| e.to_string())?;
    hasher.update(&buffer);
    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}
// */

//*
#[tauri::command]
fn delete_file(path: String) -> bool{
    std::fs::remove_file(path).is_ok()
}
// */

//*
#[tauri::command]
fn move_file(src: String, dest: String) -> bool{
    std::fs::rename(src, dest).is_ok()
}
// */

fn main() {
    /*
    file_cleaning_lib::run()
    // */
    /*
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    // */
    //*
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            scan_files_1,
            get_file_hash_1,
            delete_file,
            move_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    // */
}
