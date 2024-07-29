#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod database;
mod models;
mod repository;
mod schema;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::file::upload_file,
            // commands::upload_file,
            // commands::teacher::count_teachers,
            // commands::teacher::get_teachers
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
