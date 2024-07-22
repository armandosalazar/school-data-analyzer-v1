use crate::repository::Repository;

#[tauri::command]
pub fn get_teachers(
    offset: Option<i64>,
    page_size: Option<i64>,
) -> Vec<crate::models::teacher::Teacher> {
    println!("Getting teachers");
    println!("Offset: {:?}", offset);
    println!("Page size: {:?}", page_size);
    let mut conn = crate::database::establish_connection();
    let mut teacher_repository = crate::repository::teacher::TeacherRepository::new(&mut conn);
    match teacher_repository.find_all(offset, page_size) {
        Ok(teachers) => teachers,
        Err(e) => {
            println!("Error getting teachers: {:?}", e);
            Vec::new()
        }
    }
}

#[tauri::command]
pub fn count_teachers() -> i64 {
    println!("Counting teachers");
    let mut conn = crate::database::establish_connection();
    let mut teacher_repository = crate::repository::teacher::TeacherRepository::new(&mut conn);
    match teacher_repository.count() {
        Ok(count) => count,
        Err(e) => {
            println!("Error counting teachers: {:?}", e);
            0
        }
    }
}
