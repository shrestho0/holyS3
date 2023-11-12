// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::{Path, PathBuf};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, check_file_or_directory])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn check_file_or_directory(path: PathBuf) -> i8 {
    println!("{:?}", path);
    if !path.exists() {
        println!("file/directory does not exist");
        return -1;
    }
    if path.is_file() {
        println!("is a file");
        // proceed to check if exists
        return 1;
    } else if path.is_dir() {
        println!("is a directory");
        return 2;
    }

    return -1;
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
