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

#[tauri::command]
pub async fn get_grades_by_student_id(student_id: i64) -> Vec<Grade> {
    get_grades_id(student_id).await.unwrap_or_else(|_| Vec::new())
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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Grade {
    id: Option<i64>,
    first_grade: Option<i64>,
    first_faults: Option<i64>,
    first_weighing: Option<i64>,
    second_grade: Option<i64>,
    second_faults: Option<i64>,
    second_weighing: Option<i64>,
    third_grade: Option<i64>,
    third_faults: Option<i64>,
    third_weighing: Option<i64>,
    subject_code: Option<String>,
    subject_name: Option<String>,
    teacher_payroll: Option<i64>,
    teacher_name: Option<String>,
    division_code: Option<i64>,
    division_name: Option<String>,
}

async fn get_grades_id(student_id: i64) -> Result<Vec<Grade>, sqlx::Error> {
    let pool = SqlitePool::connect("database.db").await?;
    sqlx::query_as!(
        Grade,
        r#"SELECT
        g.id,
        g.first_grade,
        g.first_faults,
        g.first_weighing,
        g.second_grade,
        g.second_faults,
        g.second_weighing,
        g.third_grade,
        g.third_faults,
        g.third_weighing,
        s.code AS `subject_code`,
        s.name AS `subject_name`,
        t.payroll AS `teacher_payroll`,
        t.name AS `teacher_name`,
        d.code AS `division_code`,
        d.name AS `division_name`
        FROM
        grades g
        INNER JOIN subjects s ON
        s.id = g.subject_id
        INNER JOIN teachers t ON
        t.id = s.teacher_id
        INNER JOIN divisions d ON
        d.id = s.division_id
        WHERE
        g.student_id = ?"#,
        student_id
    ).fetch_all(&pool).await
}