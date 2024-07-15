use polars::prelude::*;

use crate::database;
use crate::models::subject::Subject;
use crate::models::teacher::Teacher;
use crate::repository::subject::SubjectRepository;
use crate::repository::teacher::TeacherRepository;
use crate::repository::Repository;

#[tauri::command]
pub fn upload_file(path: &str) {
    let df: LazyFrame = LazyCsvReader::new(path).finish().unwrap();
    let mut conn = database::establish_connection();
    // let mut subject_repository = SubjectRepository::new(&mut conn);

    // match create_subjects(&df, &mut subject_repository) {
    //     Ok(_) => println!("Subjects created successfully"),
    //     Err(e) => println!("Error creating subjects: {:?}", e),
    // }

    let mut teacher_repository = TeacherRepository::new(&mut conn);
    match create_teachers(&df, &mut teacher_repository) {
        Ok(_) => println!("Teachers created successfully"),
        Err(e) => println!("Error creating teachers: {:?}", e),
    }
}

#[allow(dead_code)]
fn create_subjects(
    df: &LazyFrame,
    repository: &mut SubjectRepository,
) -> Result<(), Box<dyn std::error::Error>> {
    let results: DataFrame = df
        .clone()
        .lazy()
        .limit(5)
        .select(&[col("clave"), col("nombre_duplicated_0").alias("nombre")])
        .collect()?;

    // let columns = results.get_columns();

    // for i in 0..columns[0].len() {
    //     repository.create(Subject::new(
    //         columns[1].get(i)?.to_string().replace("\"", ""),
    //         columns[0].get(i)?.to_string().replace("\"", ""),
    //     ));
    // }

    Ok(())
}

#[allow(dead_code)]
fn create_teachers(
    df: &LazyFrame,
    repository: &mut TeacherRepository,
) -> Result<(), Box<dyn std::error::Error>> {
    let result = df
        .clone()
        .lazy()
        .select(&[col("nomina"), col("nombre_duplicated_1").alias("nombre")])
        .group_by([col("nomina")])
        .agg([col("nombre").unique().first()])
        .sort(["nomina"], Default::default())
        .collect()?;
    let columns = result.get_columns();

    for i in 0..columns[0].len() {
        match repository.create(Teacher::new(
            columns[0].get(i)?.to_string().parse()?,
            columns[1].get(i)?.to_string().replace("\"", ""),
        )) {
            Ok(_) => println!("Teacher created successfully"),
            Err(e) => println!("Error creating teacher: {:?}", e),
        }
    }

    Ok(())
}
