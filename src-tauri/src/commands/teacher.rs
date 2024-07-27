use crate::repository::Repository;

#[tauri::command]
pub fn get_teachers(
    offset: Option<i64>,
    page_size: Option<i64>,
    sort_field: Option<String>,
    sort_order: Option<i64>,
    filters: Option<String>,
) -> Vec<crate::models::teacher::Teacher> {
    println!("Getting teachers: offset: {:?}, page_size: {:?}, sort_field: {:?}, sort_order: {:?}, filters: {:?}", offset, page_size, sort_field, sort_order, filters);

    let mut conn = crate::database::establish_connection();
    let mut teacher_repository = crate::repository::teacher::TeacherRepository::new(&mut conn);

    teacher_repository.find_all(offset, page_size, sort_field, sort_order, filters).unwrap_or_else(|e| {
        println!("Error getting teachers: {:?}", e);
        Vec::new()
    })
}

#[tauri::command]
pub fn count_teachers() -> i64 {
    println!("Counting teachers");
    let mut conn = crate::database::establish_connection();
    let mut teacher_repository = crate::repository::teacher::TeacherRepository::new(&mut conn);
    teacher_repository.count().unwrap_or_else(|e| {
        println!("Error counting teachers: {:?}", e);
        0
    })
}
