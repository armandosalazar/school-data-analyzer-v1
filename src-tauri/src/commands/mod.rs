use polars::prelude::*;

use crate::database;
use crate::models::division::Division;
use crate::models::teacher::Teacher;
use crate::repository::division::DivisionRepository;
use crate::repository::teacher::TeacherRepository;
use crate::repository::Repository;

#[tauri::command]
pub fn upload_file(path: &str) {
    let mut schema: Schema = Schema::new();
    schema.with_column("nomina".into(), DataType::Int32);
    schema.with_column("division".into(), DataType::Int32);

    let df: LazyFrame = LazyCsvReader::new(path)
        .with_dtype_overwrite(Some(Arc::new(schema)))
        .finish()
        .unwrap();
    let mut conn = database::establish_connection();

    let mut teacher_repository = TeacherRepository::new(&mut conn);
    match create_teachers(&df, &mut teacher_repository) {
        Ok(_) => println!("Teachers created successfully"),
        Err(e) => println!("Error creating teachers: {:?}", e),
    }
    let mut division_repository = DivisionRepository::new(&mut conn);
    match create_divisions(&df, &mut division_repository) {
        Ok(_) => println!("Divisions created successfully"),
        Err(e) => println!("Error creating divisions: {:?}", e),
    }
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

    for i in 0..result.height() {
        match repository.create(Teacher::new(
            result.column("nomina")?.i32()?.get(i).unwrap(),
            result.column("nombre")?.str()?.get(i).unwrap().to_string(),
        )) {
            Ok(_) => continue,
            Err(e) => println!("Error creating teacher: {:?}", e),
        }
    }

    Ok(())
}

#[allow(dead_code)]
fn create_divisions(
    df: &LazyFrame,
    repository: &mut DivisionRepository,
) -> Result<(), Box<dyn std::error::Error>> {
    let result = df
        .clone()
        .lazy()
        .select(&[col("division"), col("academia")])
        .group_by([col("division")])
        .agg([col("academia").unique()])
        .explode(["academia"])
        .sort(["division"], Default::default())
        .collect()?;

    for i in 0..result.height() {
        match repository.create(Division::new(
            result.column("division")?.i32()?.get(i).unwrap(),
            result
                .column("academia")?
                .str()?
                .get(i)
                .unwrap()
                .to_string(),
        )) {
            Ok(_) => continue,
            Err(e) => println!("Error creating division: {:?}", e),
        }
    }

    Ok(())
}
