use diesel::prelude::*;
use polars::prelude::*;

use crate::database;
use crate::models::division::Division;
use crate::models::subject::Subject;
use crate::models::teacher::Teacher;
use crate::repository::division::DivisionRepository;
use crate::repository::subject::SubjectRepository;
use crate::repository::teacher::TeacherRepository;
use crate::repository::speciality::SpecialityRepository;
use crate::repository::Repository;

pub mod teacher;

#[tauri::command]
pub fn upload_file(path: &str) {
    let mut schema: Schema = Schema::new();
    schema.with_column("nomina".into(), DataType::Int32);
    schema.with_column("division".into(), DataType::Int32);
    schema.with_column("especialidad".into(), DataType::Int32);

    let df: LazyFrame = LazyCsvReader::new(path)
        .with_dtype_overwrite(Some(Arc::new(schema)))
        .finish()
        .unwrap();

    let mut conn = database::establish_connection();

    // let mut teacher_repository = TeacherRepository::new(&mut conn);
    // match create_teachers(&df, &mut teacher_repository) {
    //     Ok(_) => println!("Teachers created successfully"),
    //     Err(e) => println!("Error creating teachers: {:?}", e),
    // }
    // let mut division_repository = DivisionRepository::new(&mut conn);
    // match create_divisions(&df, &mut division_repository) {
    //     Ok(_) => println!("Divisions created successfully"),
    //     Err(e) => println!("Error creating divisions: {:?}", e),
    // }
    // let mut subject_repository = SubjectRepository::new(&mut conn);
    // match create_subjects(&df, &mut subject_repository) {
    //     Ok(_) => println!("Subjects created successfully"),
    //     Err(e) => println!("Error creating subjects: {:?}", e),
    // }
    // let mut speciality_repository = SpecialityRepository::new(&mut conn);
    // match create_specialities(&df, &mut speciality_repository) {
    //     Ok(_) => println!("Specialities created successfully"),
    //     Err(e) => println!("Error creating specialities: {:?}", e),
    // }
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

#[allow(dead_code)]
fn create_subjects(
    df: &LazyFrame,
    repository: &mut SubjectRepository,
) -> Result<(), Box<dyn std::error::Error>> {
    let result = df
        .clone()
        .lazy()
        .select(&[
            col("clave"),
            col("nombre_duplicated_0").alias("nombre"),
            col("division"),
            col("academia"),
            col("nomina"),
            col("nombre_duplicated_1").alias("nombre_maestro"),
        ])
        .group_by([col("clave"), col("nombre_maestro")])
        .agg([
            col("nombre").unique().first(),
            col("division").unique().first(),
            col("academia").unique().first(),
            col("nomina").unique().first(),
        ])
        .sort(["clave"], Default::default())
        .collect()?;

    for i in 0..result.height() {
        let teacher_id: Option<i32> = crate::schema::teachers::dsl::teachers
            .filter(
                crate::schema::teachers::payfoll.eq(result
                    .column("nomina")?
                    .i32()?
                    .get(i)
                    .unwrap()),
            )
            .or_filter(
                crate::schema::teachers::name.eq(result
                    .column("nombre_maestro")?
                    .str()?
                    .get(i)
                    .unwrap()),
            )
            .select(crate::schema::teachers::id)
            .first::<Option<i32>>(repository.conn)?;

        let division_id: Option<i32> = crate::schema::divisions::table
            .filter(
                crate::schema::divisions::code.eq(result
                    .column("division")?
                    .i32()?
                    .get(i)
                    .unwrap()),
            )
            .or_filter(
                crate::schema::divisions::academy.eq(result
                    .column("academia")?
                    .str()?
                    .get(i)
                    .unwrap()),
            )
            .select(crate::schema::divisions::id)
            .first::<Option<i32>>(repository.conn)?;

        match repository.create(Subject::new(
            teacher_id.unwrap(),
            division_id.unwrap(),
            result.column("clave")?.str()?.get(i).unwrap().to_string(),
            result.column("nombre")?.str()?.get(i).unwrap().to_string(),
        )) {
            Ok(_) => continue,
            Err(e) => println!("Error creating subject: {:?}", e),
        }
    }

    Ok(())
}

#[allow(dead_code)]
fn create_specialities(
    df: &LazyFrame,
    repository: &mut SpecialityRepository,
) -> Result<(), Box<dyn std::error::Error>> {
    let result = df
        .clone()
        .lazy()
        .select(&[col("especialidad").alias("code"), col("nombre").alias("name")])
        .group_by([col("code")])
        .agg([col("name").unique()])
        .sort(["code"], Default::default())
        .explode(["name"])
        .collect()?;

    for i in 0..result.height() {
        match repository.create(crate::models::speciality::Speciality::new(
            result.column("code")?.i32()?.get(i).unwrap(),
            result.column("name")?.str()?.get(i).unwrap().to_string(),
        )) {
            Ok(_) => continue,
            Err(e) => println!("Error creating speciality: {:?}", e),
        }
    }

    Ok(())
}