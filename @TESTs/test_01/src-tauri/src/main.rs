// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn command(someshit: &str) {
    println!("some_command was called from frontend with {someshit}");
}

#[tauri::command]
fn check_file_or_directory(path: PathBuf) -> i8 {
    if path.is_file() {
        println!("is a file");
        return 0;
    } else if path.is_dir() {
        println!("is a directory");
        return 1;
    } else {
        println!("this shit does not exist");
        return -1;
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            command,
            greet,
            check_file_or_directory
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
