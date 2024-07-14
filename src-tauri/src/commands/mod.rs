use polars::prelude::*;

use crate::database;
use crate::models::subject::Subject;
use crate::repository::subject::SubjectRepository;
use crate::repository::Repository;

#[tauri::command]
pub fn upload_file(path: &str) {
    let df: LazyFrame = LazyCsvReader::new(path).finish().unwrap();
    let mut conn = database::establish_connection();
    let mut subject_repository = SubjectRepository::new(&mut conn);

    match create_subjects(&df, &mut subject_repository) {
        Ok(_) => println!("Subjects created successfully"),
        Err(e) => println!("Error creating subjects: {:?}", e),
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

    let columns = results.get_columns();

    for i in 0..columns[0].len() {
        let code = columns[0].get(i)?.to_string().replace("\"", "");
        let name = columns[1].get(i)?.to_string().replace("\"", "");

        let subject = Subject::new(name.as_str(), code.as_str());
    }

    Ok(())
}
