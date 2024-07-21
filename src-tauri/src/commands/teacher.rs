use crate::repository::Repository;

#[tauri::command]
pub fn get_teachers() -> Vec<crate::models::teacher::Teacher> {
    let mut conn = crate::database::establish_connection();
    let mut teacher_repository = crate::repository::teacher::TeacherRepository::new(&mut conn);
    match teacher_repository.find_all() {
        Ok(teachers) => teachers,
        Err(e) => {
            println!("Error getting teachers: {:?}", e);
            Vec::new()
        }
    }
}
