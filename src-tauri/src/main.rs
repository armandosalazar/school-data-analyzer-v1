// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(_name: &str) -> String {
    let conn = &mut establish_connection();

    println!("Listing tests");

    use crate::models::Test;
    use crate::schema::tests::dsl::*;

    tests.load::<Test>(conn)
        .expect("Error loading tests")
        .iter()
        .for_each(|test| {
            println!("{}: {} {}", test.id, test.name, test.created_at);
        });



    format!("Hello, You've been greeted from Rust!")
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

use diesel::prelude::*;
use std::env;
use dotenvy::dotenv;

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

mod schema;
mod models {
    use diesel::Queryable;
    use crate::schema::tests;

    #[derive(Queryable)]
    #[diesel(table_name = tests)]
    pub struct Test {
        pub id: i32,
        pub name: String,
        pub created_at: chrono::NaiveDateTime,
    }
}