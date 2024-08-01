use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::sqlite::SqlitePool;


#[tauri::command]
pub async fn get_students(limit: Option<i64>, offset: Option<i64>, filters: Option<String>) -> Vec<Student> {
    println!("{:?}", filters);
    read_students(limit.unwrap_or(10), offset.unwrap_or(0), filters.unwrap_or("[]".to_string())).await.unwrap_or_else(|_| Vec::new())
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
#[derive(FromRow)]
pub struct Student {
    id: Option<i64>,
    register: Option<i64>,
    name: Option<String>,
    #[serde(rename = "type")]
    type_: Option<String>,
}

#[derive(Debug)]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Filter {
    name: Option<String>,
    value: Option<String>,
    match_mode: Option<String>,
}
async fn read_students(limit: i64, offset: i64, filters: String) -> Result<Vec<Student>, sqlx::Error> {
    let pool = SqlitePool::connect("database.db").await?;

    let mut query_builder = sqlx::QueryBuilder::new(r#"
    SELECT s.id, s.register, s.name, s.type as type_
    FROM students s
    "#);

    let filters: Vec<Filter> = serde_json::from_str(&filters).unwrap_or_else(|_| Vec::new());
    let mut where_added = false;
    for filter in filters {
        if filter.value.is_some() {
            if !where_added {
                query_builder.push(" WHERE ");
                where_added = true;
            } else {
                query_builder.push(" AND ");
            }
            match filter.match_mode.unwrap().as_str() {
                "startsWith" => {
                    query_builder.push(format!(" s.{} LIKE ", filter.name.unwrap()));
                    query_builder.push_bind(format!("{}%", filter.value.unwrap()));
                }
                "contains" => {
                    query_builder.push(format!(" s.{} LIKE ", filter.name.unwrap()));
                    query_builder.push_bind(format!("%{}%", filter.value.unwrap()));
                }
                "notContains" => {
                    query_builder.push(format!(" s.{} NOT LIKE ", filter.name.unwrap()));
                    query_builder.push_bind(format!("%{}%", filter.value.unwrap()));
                }
                "endsWith" => {
                    query_builder.push(format!(" s.{} LIKE ", filter.name.unwrap()));
                    query_builder.push_bind(format!("%{}", filter.value.unwrap()));
                }
                "equals" => {
                    query_builder.push(format!(" s.{} = ", filter.name.unwrap()));
                    query_builder.push_bind(filter.value.unwrap());
                }
                _ => {}
            }
        }
    }

    query_builder.push(" LIMIT ").push_bind(limit);
    query_builder.push(" OFFSET ").push_bind(offset);

    let sql = query_builder.sql();
    println!("{:?}", sql.to_string());

    let query = query_builder.build_query_as::<Student>();

    query.fetch_all(&pool).await
}

async fn count_test() -> Result<i64, sqlx::Error> {
    let pool = SqlitePool::connect("database.db").await?;

    sqlx::query_scalar("SELECT COUNT(*) FROM students").fetch_one(&pool).await
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Grade {
    id: Option<i64>,
    first_grade: Option<f64>,
    first_faults: Option<i64>,
    first_weighing: Option<i64>,
    second_grade: Option<f64>,
    second_faults: Option<i64>,
    second_weighing: Option<i64>,
    third_grade: Option<f64>,
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