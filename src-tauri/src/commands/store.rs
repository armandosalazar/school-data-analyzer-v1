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

fn create_specialities(
    df: &LazyFrame,
    repository: &mut crate::repository::speciality::SpecialityRepository,
) -> Result<String, Box<dyn std::error::Error>> {
    let specialities = df
        .clone()
        .lazy()
        .select(&[
            col("especialidad").alias("code").cast(DataType::Int32),
            col("nombre").alias("name"),
        ])
        .group_by([col("code")])
        .agg(vec![col("name").unique()])
        .explode(["name"])
        .sort(["code"], Default::default())
        .collect()?;

    for i in 0..specialities.height() {
        match repository.create(crate::models::speciality::Speciality::new(
            specialities.column("code")?.i32()?.get(i).unwrap(),
            specialities.column("name")?.str()?.get(i).unwrap().to_string(),
        )) {
            Ok(_) => continue,
            Err(e) => Err(e)?,
        }
    }

    Ok("Specialities created".to_string())
}

fn create_students(
    df: &LazyFrame,
    repository: &mut crate::repository::student::StudentRepository,
) -> Result<String, Box<dyn std::error::Error>> {
    let students = df
        .clone()
        .lazy()
        .select(&[
            col("registro").alias("register").cast(DataType::Int32),
            col("nombre_completo").alias("name"),
            col("tipo").alias("type"),
            col("estado").alias("status"),
            col("semestre").alias("semester").cast(DataType::Int32),
            col("grupo").alias("group"),
            col("turno").alias("shift"),
            col("nivel").alias("level"),
            col("especialidad").alias("speciality_code").cast(DataType::Int32),
            col("nombre").alias("speciality_name"),
        ])
        .group_by([col("registro")])
        .agg([
            col("name").unique().first(),
            col("type").unique().first(),
            col("status").unique().first(),
            col("semester").unique().first(),
            col("group").unique().first(),
            col("shift").unique().first(),
            col("level").unique().first(),
            col("speciality_code").unique().first(),
            col("speciality_name").unique().first(),
        ])
        .sort(["registro"], Default::default())
        .collect()?;

    for i in 0..students.height() {
        let speciality_id: Option<i32> = crate::schema::specialities::table.filter(crate::schema::specialities::code.eq(students.column("speciality_code")?.i32()?.get(i).unwrap()))
            .or_filter(crate::schema::specialities::name.eq(students.column("speciality_name")?.str()?.get(i).unwrap()))
            .select(crate::schema::specialities::id)
            .first::<Option<i32>>(repository.conn)?;

        match repository.create(crate::models::student::Student::new(
            speciality_id.unwrap(),
            students.column("register")?.i32()?.get(i).unwrap(),
            students.column("name")?.str()?.get(i).unwrap().to_string(),
            students.column("type")?.str()?.get(i).unwrap().to_string(),
            students.column("status")?.str()?.get(i).unwrap().to_string(),
            students.column("semester")?.i32()?.get(i).unwrap(),
            students.column("group")?.str()?.get(i).unwrap().to_string(),
            students.column("shift")?.str()?.get(i).unwrap().to_string(),
            students.column("level")?.str()?.get(i).unwrap().to_string(),
        )) {
            Ok(_) => continue,
            Err(e) => Err(e)?,
        }
    }

    Ok("Students created".to_string())
}


pub async fn create_grades(
    lf: &LazyFrame,
    // repository: &mut crate::repository::grade::GradeRepository,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut conn = crate::database::establish_connection();
    let df = lf.clone().limit(0).collect()?;

    let column_names = df.get_column_names();
    let mut select_columns = Vec::new();
    select_columns.push(col("registro").alias("register").cast(DataType::Int32));
    select_columns.push(col("nombre_completo").alias("name"));
    select_columns.push(col("clave").alias("code"));
    select_columns.push(col("nombre_duplicated_0").alias("subject_name"));
    select_columns.push(col("estatus_materia").alias("subject_status"));
    select_columns.push(col("nomina").alias("payroll").cast(DataType::Int32));
    let mut agg_columns = Vec::new();
    agg_columns.push(col("name").unique().first());
    agg_columns.push(col("subject_name").unique().first());
    agg_columns.push(col("subject_status").unique().first());
    agg_columns.push(col("payroll").unique().first());

    let required_columns = vec![
        "calificacion1",
        "faltas1",
        "ponderacion1",
        "calificacion2",
        "faltas2",
        "ponderacion2",
        "calificacion3",
        "faltas3",
        "ponderacion3",
    ];

    let mut last_column = String::new();

    for column in required_columns {
        if column_names.contains(&column) {
            select_columns.push(col(column).cast(DataType::Int32));
            agg_columns.push(col(column).unique().first());
            last_column = column.to_string();
        }
    }


    println!("{:?}", select_columns);
    let grades = lf
        .clone()
        .lazy()
        .select(&select_columns)
        .group_by([col("register"), col("code")])
        .agg(agg_columns)
        .sort(["register"], Default::default())
        .collect()?;
    println!("{:?}", grades);
    println!("{:?}", last_column);

    match last_column.as_str() {
        "ponderacion1" => {
            for i in 0..grades.height() {
                let student_id: Option<i32> = get_student_id(
                    grades.column("register")?.i32()?.get(i).unwrap(),
                    &mut conn,
                );

                let teacher_id: Option<i32> = get_teacher_id(
                    grades.column("payroll")?.i32()?.get(i).unwrap(),
                    &mut conn,
                );

                let subject_id: Option<i32> = get_subject_id(
                    grades.column("code")?.str()?.get(i).unwrap(),
                    teacher_id.unwrap(),
                    &mut conn,
                );


                match crate::repository::grade::GradeRepository::new(&mut conn).create(crate::models::grade::Grade {
                    id: None,
                    student_id,
                    subject_id,
                    first_grade: grades.column("calificacion1")?.i32()?.get(i),
                    second_grade: None,
                    third_grade: None,
                    first_faults: grades.column("faltas1")?.i32()?.get(i),
                    second_faults: None,
                    third_faults: None,
                    first_weighing: grades.column("ponderacion1")?.i32()?.get(i),
                    second_weighing: None,
                    third_weighing: None,
                }) {
                    Ok(_) => continue,
                    Err(e) => Err(e)?,
                }
            }
        }
        "ponderacion2" => {
            for i in 0..grades.height() {
                let student_id: Option<i32> = get_student_id(
                    grades.column("register")?.i32()?.get(i).unwrap(),
                    &mut conn,
                );

                let teacher_id: Option<i32> = get_teacher_id(
                    grades.column("payroll")?.i32()?.get(i).unwrap(),
                    &mut conn,
                );

                let subject_id: Option<i32> = get_subject_id(
                    grades.column("code")?.str()?.get(i).unwrap(),
                    teacher_id.unwrap(),
                    &mut conn,
                );


                match crate::repository::grade::GradeRepository::new(&mut conn).create(crate::models::grade::Grade {
                    id: None,
                    student_id,
                    subject_id,
                    first_grade: grades.column("calificacion1")?.i32()?.get(i),
                    second_grade: grades.column("calificacion2")?.i32()?.get(i),
                    third_grade: None,
                    first_faults: grades.column("faltas1")?.i32()?.get(i),
                    second_faults: grades.column("faltas2")?.i32()?.get(i),
                    third_faults: None,
                    first_weighing: grades.column("ponderacion1")?.i32()?.get(i),
                    second_weighing: grades.column("ponderacion2")?.i32()?.get(i),
                    third_weighing: None,
                }) {
                    Ok(_) => continue,
                    Err(e) => Err(e)?,
                }
            }
        }
        "ponderacion3" => {
            for i in 0..grades.height() {
                let student_id: Option<i32> = get_student_id(
                    grades.column("register")?.i32()?.get(i).unwrap(),
                    &mut conn,
                );

                let teacher_id: Option<i32> = get_teacher_id(
                    grades.column("payroll")?.i32()?.get(i).unwrap(),
                    &mut conn,
                );

                let subject_id: Option<i32> = get_subject_id(
                    grades.column("code")?.str()?.get(i).unwrap(),
                    teacher_id.unwrap(),
                    &mut conn,
                );


                match crate::repository::grade::GradeRepository::new(&mut conn).create(crate::models::grade::Grade {
                    id: None,
                    student_id,
                    subject_id,
                    first_grade: grades.column("calificacion1")?.i32()?.get(i),
                    second_grade: grades.column("calificacion2")?.i32()?.get(i),
                    third_grade: grades.column("calificacion3")?.i32()?.get(i),
                    first_faults: grades.column("faltas1")?.i32()?.get(i),
                    second_faults: grades.column("faltas2")?.i32()?.get(i),
                    third_faults: grades.column("faltas3")?.i32()?.get(i),
                    first_weighing: grades.column("ponderacion1")?.i32()?.get(i),
                    second_weighing: grades.column("ponderacion2")?.i32()?.get(i),
                    third_weighing: grades.column("ponderacion3")?.i32()?.get(i),
                }) {
                    Ok(_) => continue,
                    Err(e) => Err(e)?,
                }
            }
        }
        _ => {
            for i in 0..grades.height() {
                let student_id: Option<i32> = get_student_id(
                    grades.column("register")?.i32()?.get(i).unwrap(),
                    &mut conn,
                );

                let teacher_id: Option<i32> = get_teacher_id(
                    grades.column("payroll")?.i32()?.get(i).unwrap(),
                    &mut conn,
                );

                let subject_id: Option<i32> = get_subject_id(
                    grades.column("code")?.str()?.get(i).unwrap(),
                    teacher_id.unwrap(),
                    &mut conn,
                );


                match crate::repository::grade::GradeRepository::new(&mut conn).create(crate::models::grade::Grade {
                    id: None,
                    student_id,
                    subject_id,
                    first_grade: None,
                    second_grade: None,
                    third_grade: None,
                    first_faults: None,
                    second_faults: None,
                    third_faults: None,
                    first_weighing: None,
                    second_weighing: None,
                    third_weighing: None,
                }) {
                    Ok(_) => continue,
                    Err(e) => Err(e)?,
                }
            }
        }
    }


    Ok("Grades created".to_string())
}

fn get_student_id(register: i32, conn: &mut SqliteConnection) -> Option<i32> {
    crate::schema::students::table
        .filter(crate::schema::students::register.eq(register))
        .select(crate::schema::students::id)
        .first::<Option<i32>>(conn)?
}
fn get_teacher_id(payroll: i32, conn: &mut SqliteConnection) -> Option<i32> {
    crate::schema::teachers::table
        .filter(crate::schema::teachers::payroll.eq(payroll))
        .select(crate::schema::teachers::id)
        .first::<Option<i32>>(conn)?
}

fn get_subject_id(code: &str, teacher_id: i32, conn: &mut SqliteConnection) -> Option<i32> {
    crate::schema::subjects::table
        .filter(crate::schema::subjects::code.eq(code).and(crate::schema::subjects::teacher_id.eq(teacher_id)))
        .select(crate::schema::subjects::id)
        .first::<Option<i32>>(conn)?
}