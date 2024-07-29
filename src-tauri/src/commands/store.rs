use diesel::prelude::*;
use polars::prelude::*;
use crate::models::division::Division;
use crate::models::subject::Subject;
use crate::models::teacher::Teacher;
use crate::repository::Repository;

pub fn crate_teachers(
    df: &LazyFrame,
    repository: &mut crate::repository::teacher::TeacherRepository<'_>,
) -> Result<String, Box<dyn std::error::Error>> {
    let teachers = df
        .clone()
        .lazy()
        .select(&[
            col("nomina")
                .alias("payroll")
                .cast(DataType::Int32),
            col("nombre_duplicated_1")
                .alias("name"),
        ])
        .group_by([col("payroll")])
        .agg(vec![col("name").unique().first()])
        .sort(&["payroll"], Default::default())
        .collect()?;

    for i in 0..teachers.height() {
        match repository.create(Teacher::new(
            teachers.column("payroll")?.i32()?.get(i).unwrap(),
            teachers.column("name")?.str()?.get(i).unwrap().to_string(),
        )) {
            Ok(_) => {}
            Err(e) => Err(e)?,
        }
    }

    Ok("Teachers created".to_string())
}

fn create_divisions(
    df: &LazyFrame,
    repository: &mut crate::repository::division::DivisionRepository,
) -> Result<String, Box<dyn std::error::Error>> {
    let divisions = df
        .clone()
        .lazy()
        .select(&[
            col("division").alias("code").cast(DataType::Int32),
            col("academia").alias("name")
        ])
        .group_by([col("code")])
        .agg(vec![col("name").unique()])
        .explode(["name"])
        .sort(["code"], Default::default())
        .collect()?;

    for i in 0..divisions.height() {
        match repository.create(Division::new(
            divisions.column("code")?.i32()?.get(i).unwrap(),
            divisions.column("name")?.str()?.get(i).unwrap().to_string(),
        )) {
            Ok(_) => continue,
            Err(e) => Err(e)?,
        }
    }

    Ok("Divisions created".to_string())
}

fn create_subjects(
    df: &LazyFrame,
    repository: &mut crate::repository::subject::SubjectRepository,
) -> Result<String, Box<dyn std::error::Error>> {
    let subjects = df
        .clone()
        .lazy()
        .select(&[
            col("clave").alias("code"),
            col("nombre_duplicated_0").alias("name"),
            col("division").alias("division_code").cast(DataType::Int32),
            col("academia").alias("academy_name"),
            col("nomina").alias("payroll").cast(DataType::Int32),
            col("nombre_duplicated_1").alias("teacher_name"),
        ])
        .group_by([col("clave"), col("teacher_name")])
        .agg(vec![
            col("name").unique().first(),
            col("division_code").unique().first(),
            col("academy_name").unique().first(),
            col("payroll").unique().first(),
        ])
        .sort(["code"], Default::default())
        .collect()?;

    for i in 0..subjects.height() {
        let teacher_id: Option<i32> = crate::schema::teachers::dsl::teachers
            .filter(crate::schema::teachers::payroll.eq(subjects.column("payroll")?.i32()?.get(i).unwrap()))
            .or_filter(crate::schema::teachers::name.eq(subjects.column("teacher_name")?.str()?.get(i).unwrap()))
            .select(crate::schema::teachers::id)
            .first::<Option<i32>>(repository.conn)?;

        let division_id: Option<i32> = crate::schema::divisions::table
            .filter(crate::schema::divisions::code.eq(subjects.column("division_code")?.i32()?.get(i).unwrap()))
            .or_filter(crate::schema::divisions::name.eq(subjects.column("academy_name")?.str()?.get(i).unwrap()))
            .select(crate::schema::divisions::id)
            .first::<Option<i32>>(repository.conn)?;

        match repository.create(Subject::new(
            teacher_id.unwrap(),
            division_id.unwrap(),
            subjects.column("code")?.str()?.get(i).unwrap().to_string(),
            subjects.column("name")?.str()?.get(i).unwrap().to_string(),
        )) {
            Ok(_) => continue,
            Err(e) => Err(e)?,
        }
    }

    Ok("Subjects created".to_string())
}
