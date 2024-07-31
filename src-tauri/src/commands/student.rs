use serde::Serialize;
use sqlx::sqlite::SqlitePool;

#[tauri::command]
pub async fn get_students() {
    get_test().await;
}

#[derive(Debug)]
#[derive(Serialize)]
pub struct Student {
    id: Option<i64>,
    register: Option<i64>,
    name: Option<String>,
}

async fn get_test() {
    let pool = SqlitePool::connect("database.db").await.unwrap();

    let students = sqlx::query_as!(
        Student,
        r#"SELECT s.id, s.register, s.name FROM students s"#
    );

    let students = students.fetch_all(&pool).await.unwrap();

    println!("{:?}", students);
}