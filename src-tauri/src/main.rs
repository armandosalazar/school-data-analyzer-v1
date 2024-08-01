#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::commands::file::upload_file;
use crate::commands::student::get_students;
use crate::commands::teacher::{count_teachers, get_teachers};

mod commands;
mod database;
mod models;
mod repository;
mod schema;
mod exp;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            upload_file,
            get_students,
            commands::student::count_students,
            commands::student::get_grades_by_student_id,
            count_teachers,
            get_teachers
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
