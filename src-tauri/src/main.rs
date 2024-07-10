#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod schema;
mod models;
mod database;
mod commands;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![commands::upload_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

