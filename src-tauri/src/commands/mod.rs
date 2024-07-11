use polars::prelude::*;

use crate::database;
use crate::repository::{Repository, Student, StudentRepository};

#[tauri::command]
pub fn upload_file(path: &str) {
    println!("@celled");
    let df = CsvReadOptions::default()
        .try_into_reader_with_file_path(Some(path.into()))
        .unwrap()
        .finish()
        .unwrap();

    let mut conn = database::establish_connection();
    let mut student_repository = StudentRepository::new(&mut conn);
    let _ = student_repository.create(Student::new(
        1,
        "Armando Salazar".to_string(),
        "type_".to_string(),
        "status".to_string(),
        "semester".to_string(),
        "group".to_string(),
        "turn".to_string(),
        "level".to_string(),
    ));

    println!("{}", df);
    println!("{}", path)
}
