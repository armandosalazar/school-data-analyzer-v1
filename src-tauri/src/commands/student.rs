use serde::Serialize;
use sqlx::sqlite::SqlitePool;

#[tauri::command]
pub async fn get_students(limit: Option<i64>, offset: Option<i64>) -> Vec<Student> {
    get_test(limit.unwrap_or(10), offset.unwrap_or(0)).await.unwrap_or_else(|_| Vec::new())
}

#[tauri::command]
pub async fn count_students() -> i64 {
    count_test().await.unwrap_or_else(|_| 0)
}

#[derive(Debug)]
#[derive(Serialize)]
pub struct Student {
    id: Option<i64>,
    register: Option<i64>,
    name: Option<String>,
    #[serde(rename = "type")]
    type_: Option<String>,
}

async fn get_test(limit: i64, offset: i64) -> Result<Vec<Student>, sqlx::Error> {
    let pool = SqlitePool::connect("database.db").await?;

    sqlx::query_as!(
        Student,
        r#"SELECT s.id, s.register, s.name, s.`type` AS type_ FROM students s LIMIT ? OFFSET ?"#,
        limit,
        offset
    ).fetch_all(&pool).await
}

async fn count_test() -> Result<i64, sqlx::Error> {
    let pool = SqlitePool::connect("database.db").await?;

    sqlx::query_scalar("SELECT COUNT(*) FROM students").fetch_one(&pool).await
}