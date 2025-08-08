/*
use std::fs;

// ディレクトリ内のファイル一覧を取得するコマンド
#[tauri::command]
pub fn get_file_list(directory_path: String) -> Result<Vec<String>, String> {
    // app: tauri::AppHandle
    // , 

    //let directory_path = app.dialog().file().blocking_pick_file();
    // return a file_path `Option`, or `None` if the user closes the dialog

    // ディレクトリを読み込む
    let entries = match fs::read_dir(&directory_path) {
        Ok(entries) => entries,
        Err(e) => return Err(format!("ディレクトリを開けませんでした: {}", e)),
    };

    // ファイル名を格納するリスト
    let mut file_names = Vec::new();

    // ディレクトリ内の各エントリを処理
    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => return Err(format!("エントリを読み込めませんでした: {}", e)),
        };

        // ファイル名を文字列として取得
        let file_name = match entry.file_name().into_string() {
            Ok(name) => name,
            Err(_) => continue, // 不正なファイル名はスキップ
        };

        file_names.push(file_name);
    }

    Ok(file_names)
}
*/

// 動作確認。
#[tauri::command]
fn test_invoke_1() -> String {
    "インボークの動作確認１。".to_string()
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        //.plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            test_invoke_1,
            greet
        ])
        //.run(tauri::generate_context!())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
