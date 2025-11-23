use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[tauri::command]
fn has_accounts() -> bool {
    false
}

#[tauri::command]
fn login_with_google() -> String {
    match Command::new("bash")
        .arg("basalt")
        .arg("auth")
        .arg("google")
        .output()
    {
        Ok(_) => "".to_string(),
        Err(error) => error.to_string(),
    }
}

#[tauri::command]
fn initialize_data_dir() -> bool {
    if let Some(home_path) = env::var("HOME").ok().map(PathBuf::from) {
        let cache_dir = home_path.join(".cache").join("basalt-gui");
        let marker_file = cache_dir.join(".initialized");
        fs::create_dir_all(&cache_dir).unwrap();
        
        if marker_file.exists() {
            true
        } else {
            fs::write(&marker_file, "").unwrap();
            false
        }
    } else {
        false
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![initialize_data_dir, has_accounts, login_with_google])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
