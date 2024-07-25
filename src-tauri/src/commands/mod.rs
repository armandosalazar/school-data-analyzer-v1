use diesel::prelude::*;
use polars::prelude::*;
use crate::database;
use crate::models::division::Division;
use crate::models::subject::Subject;
use crate::models::teacher::Teacher;
use crate::repository::division::DivisionRepository;
use crate::repository::grade::GradeRepository;
use crate::repository::subject::SubjectRepository;
use crate::repository::teacher::TeacherRepository;
use crate::repository::speciality::SpecialityRepository;
use crate::repository::Repository;
use crate::repository::student::StudentRepository;

pub mod teacher;

#[tauri::command]
pub fn upload_file(path: &str) {
    let mut schema: Schema = Schema::new();
    schema.with_column("nomina".into(), DataType::Int32);
    schema.with_column("division".into(), DataType::Int32);
    schema.with_column("especialidad".into(), DataType::Int32);
    schema.with_column("registro".into(), DataType::Int32);
    schema.with_column("semestre".into(), DataType::Int32);
    schema.with_column("calificacion1".into(), DataType::Int32);
    schema.with_column("calificacion2".into(), DataType::Int32);
    schema.with_column("calificacion3".into(), DataType::Int32);
    schema.with_column("faltas1".into(), DataType::Int32);
    schema.with_column("faltas2".into(), DataType::Int32);
    schema.with_column("faltas3".into(), DataType::Int32);
    schema.with_column("ponderacion1".into(), DataType::Int32);
    schema.with_column("ponderacion2".into(), DataType::Int32);
    schema.with_column("ponderacion3".into(), DataType::Int32);

    let df: LazyFrame = LazyCsvReader::new(path)
        .with_dtype_overwrite(Some(Arc::new(schema)))
        .finish()
        .unwrap();

    let mut conn = database::establish_connection();

    match create_teachers(&df, &mut TeacherRepository::new(&mut conn)) {
        Ok(_) => println!("Teachers created successfully"),
        Err(e) => println!("Error creating teachers: {:?}", e),
    }
    match create_divisions(&df, &mut DivisionRepository::new(&mut conn)) {
        Ok(_) => println!("Divisions created successfully"),
        Err(e) => println!("Error creating divisions: {:?}", e),
    }
    match create_subjects(&df, &mut SubjectRepository::new(&mut conn)) {
        Ok(_) => println!("Subjects created successfully"),
        Err(e) => println!("Error creating subjects: {:?}", e),
    }
    match create_specialities(&df, &mut SpecialityRepository::new(&mut conn)) {
        Ok(_) => println!("Specialities created successfully"),
        Err(e) => println!("Error creating specialities: {:?}", e),
    }
    match create_students(&df, &mut StudentRepository::new(&mut conn)) {
        Ok(_) => println!("Students created successfully"),
        Err(e) => println!("Error creating students: {:?}", e),
    }
    match create_grades(&df, &mut GradeRepository::new(&mut conn)) {
        Ok(_) => println!("Grades created successfully"),
        Err(e) => println!("Error creating grades: {:?}", e),
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
                crate::schema::teachers::payroll.eq(result
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
                crate::schema::divisions::name.eq(result
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

#[allow(dead_code)]
fn create_students(
    df: &LazyFrame,
    repository: &mut StudentRepository,
) -> Result<(), Box<dyn std::error::Error>> {
    let result = df.clone()
        .lazy()
        .select(&[
            col("registro"),
            col("nombre_completo").alias("nombre"),
            col("tipo"),
            col("estado"),
            col("semestre"),
            col("grupo"),
            col("turno"),
            col("nivel"),
            col("especialidad"),
            col("nombre").alias("nombre_especialidad"),
        ])
        .group_by([col("registro")])
        .agg([
            col("nombre").unique().first(),
            col("tipo").unique().first(),
            col("estado").unique().first(),
            col("semestre").unique().first(),
            col("grupo").unique().first(),
            col("turno").unique().first(),
            col("nivel").unique().first(),
            col("especialidad").unique().first(),
            col("nombre_especialidad").unique().first(),
        ])
        .sort(["registro"], Default::default())
        .collect()?;

    for i in 0..result.height() {
        let speciality_id: Option<i32> = crate::schema::specialities::table
            .filter(
                crate::schema::specialities::code.eq(result
                    .column("especialidad")?
                    .i32()?
                    .get(i)
                    .unwrap()),
            )
            .or_filter(
                crate::schema::specialities::name.eq(result
                    .column("nombre_especialidad")?
                    .str()?
                    .get(i)
                    .unwrap()),
            )
            .select(crate::schema::specialities::id)
            .first::<Option<i32>>(repository.conn)?;

        match repository.create(crate::models::student::Student::new(
            speciality_id.unwrap(),
            result.column("registro")?.i32()?.get(i).unwrap(),
            result.column("nombre")?.str()?.get(i).unwrap().to_string(),
            result.column("tipo")?.str()?.get(i).unwrap().to_string(),
            result.column("estado")?.str()?.get(i).unwrap().to_string(),
            result.column("semestre")?.i32()?.get(i).unwrap(),
            result.column("grupo")?.str()?.get(i).unwrap().to_string(),
            result.column("turno")?.str()?.get(i).unwrap().to_string(),
            result.column("nivel")?.str()?.get(i).unwrap().to_string(),
        )) {
            Ok(_) => continue,
            Err(e) => println!("Error creating student: {:?}", e),
        }
    }

    Ok(())
}

#[allow(dead_code)]
fn create_grades(df: &LazyFrame, repository: &mut crate::repository::grade::GradeRepository) -> Result<(), Box<dyn std::error::Error>> {
    if df.clone().schema()?.index_of("calificacion1").is_some() & df.clone().schema()?.index_of("calificacion2").is_some() & df.clone().schema()?.index_of("calificacion3").is_some() {
        let result = df.clone()
            .lazy()
            .select(
                &[
                    col("registro"),
                    col("nombre_completo").alias("nombre_alumno"),
                    col("clave"),
                    col("nombre_duplicated_0").alias("nombre_materia"),
                    col("estatus_materia"),
                    col("nomina").alias("nomina_maestro"),
                    col("calificacion1").alias("calificacion_primer_parcial"),
                    col("faltas1").alias("faltas_primer_parcial"),
                    col("ponderacion1").alias("ponderacion_primer_parcial"),
                    col("calificacion2").alias("calificacion_segundo_parcial"),
                    col("faltas2").alias("faltas_segundo_parcial"),
                    col("ponderacion2").alias("ponderacion_segundo_parcial"),
                    col("calificacion3").alias("calificacion_tercer_parcial"),
                    col("faltas3").alias("faltas_tercer_parcial"),
                    col("ponderacion3").alias("ponderacion_tercer_parcial"),
                ]
            )
            .group_by([col("registro"), col("clave")])
            .agg(
                [
                    col("nombre_alumno").unique().first(),
                    col("nombre_materia").unique().first(),
                    col("estatus_materia").unique().first(),
                    col("nomina_maestro").unique().first(),
                    col("calificacion_primer_parcial").unique().first(),
                    col("faltas_primer_parcial").unique().first(),
                    col("ponderacion_primer_parcial").unique().first(),
                    col("calificacion_segundo_parcial").unique().first(),
                    col("faltas_segundo_parcial").unique().first(),
                    col("ponderacion_segundo_parcial").unique().first(),
                    col("calificacion_tercer_parcial").unique().first(),
                    col("faltas_tercer_parcial").unique().first(),
                    col("ponderacion_tercer_parcial").unique().first(),
                ]
            ).sort(["registro"], Default::default())
            .collect()?;

        for i in 0..result.height() {
            let student_id: Option<i32> = crate::schema::students::table
                .filter(
                    crate::schema::students::register.eq(result
                        .column("registro")?
                        .i32()?
                        .get(i)
                        .unwrap()),
                )
                .select(crate::schema::students::id)
                .first::<Option<i32>>(repository.conn)?;
            let teacher_id: Option<i32> = crate::schema::teachers::table
                .filter(
                    crate::schema::teachers::payroll.eq(result
                        .column("nomina_maestro")?
                        .i32()?
                        .get(i)
                        .unwrap()),
                )
                .select(crate::schema::teachers::id)
                .first::<Option<i32>>(repository.conn)?;
            let subject_id: Option<i32> = crate::schema::subjects::table
                .filter(
                    crate::schema::subjects::code.eq(result
                        .column("clave")?
                        .str()?
                        .get(i)
                        .unwrap()).and(
                        crate::schema::subjects::teacher_id.eq(teacher_id.unwrap())
                    ),
                )
                .select(crate::schema::subjects::id)
                .first::<Option<i32>>(repository.conn)?;

            match repository.create(crate::models::grade::Grade {
                id: None,
                student_id,
                subject_id,
                first_grade: result.column("calificacion_primer_parcial")?.i32()?.get(i),
                second_grade: result.column("calificacion_segundo_parcial")?.i32()?.get(i),
                third_grade: result.column("calificacion_tercer_parcial")?.i32()?.get(i),
                first_faults: result.column("faltas_primer_parcial")?.i32()?.get(i),
                second_faults: result.column("faltas_segundo_parcial")?.i32()?.get(i),
                third_faults: result.column("faltas_tercer_parcial")?.i32()?.get(i),
                first_weighing: result.column("ponderacion_primer_parcial")?.i32()?.get(i),
                second_weighing: result.column("ponderacion_segundo_parcial")?.i32()?.get(i),
                third_weighing: result.column("ponderacion_tercer_parcial")?.i32()?.get(i),
            }) {
                Ok(_) => continue,
                Err(e) => println!("Error creating grade: {:?}", e),
            }
        }
    }
    if df.clone().schema()?.index_of("calificacion1").is_some() & df.clone().schema()?.index_of("calificacion2").is_some() {
        let result = df.clone()
            .lazy()
            .select(
                &[
                    col("registro"),
                    col("nombre_completo").alias("nombre_alumno"),
                    col("clave"),
                    col("nombre_duplicated_0").alias("nombre_materia"),
                    col("estatus_materia"),
                    col("nomina").alias("nomina_maestro"),
                    col("calificacion1").alias("calificacion_primer_parcial"),
                    col("faltas1").alias("faltas_primer_parcial"),
                    col("ponderacion1").alias("ponderacion_primer_parcial"),
                    col("calificacion2").alias("calificacion_segundo_parcial"),
                    col("faltas2").alias("faltas_segundo_parcial"),
                    col("ponderacion2").alias("ponderacion_segundo_parcial"),
                ]
            )
            .group_by([col("registro"), col("clave")])
            .agg(
                [
                    col("nombre_alumno").unique().first(),
                    col("nombre_materia").unique().first(),
                    col("estatus_materia").unique().first(),
                    col("nomina_maestro").unique().first(),
                    col("calificacion_primer_parcial").unique().first(),
                    col("faltas_primer_parcial").unique().first(),
                    col("ponderacion_primer_parcial").unique().first(),
                    col("calificacion_segundo_parcial").unique().first(),
                    col("faltas_segundo_parcial").unique().first(),
                    col("ponderacion_segundo_parcial").unique().first(),
                ]
            )
            .sort(["registro"], Default::default())
            .collect()?;

        for i in 0..result.height() {
            let student_id: Option<i32> = crate::schema::students::table
                .filter(
                    crate::schema::students::register.eq(result
                        .column("registro")?
                        .i32()?
                        .get(i)
                        .unwrap()),
                )
                .select(crate::schema::students::id)
                .first::<Option<i32>>(repository.conn)?;
            let teacher_id: Option<i32> = crate::schema::teachers::table
                .filter(
                    crate::schema::teachers::payroll.eq(result
                        .column("nomina_maestro")?
                        .i32()?
                        .get(i)
                        .unwrap()),
                )
                .select(crate::schema::teachers::id)
                .first::<Option<i32>>(repository.conn)?;
            let subject_id: Option<i32> = crate::schema::subjects::table
                .filter(
                    crate::schema::subjects::code.eq(result
                        .column("clave")?
                        .str()?
                        .get(i)
                        .unwrap()).and(
                        crate::schema::subjects::teacher_id.eq(teacher_id.unwrap())
                    ),
                )
                .select(crate::schema::subjects::id)
                .first::<Option<i32>>(repository.conn)?;

            match repository.create(crate::models::grade::Grade {
                id: None,
                student_id,
                subject_id,
                first_grade: result.column("calificacion_primer_parcial")?.i32()?.get(i),
                second_grade: result.column("calificacion_segundo_parcial")?.i32()?.get(i),
                third_grade: None,
                first_faults: result.column("faltas_primer_parcial")?.i32()?.get(i),
                second_faults: result.column("faltas_segundo_parcial")?.i32()?.get(i),
                third_faults: None,
                first_weighing: result.column("ponderacion_primer_parcial")?.i32()?.get(i),
                second_weighing: result.column("ponderacion_segundo_parcial")?.i32()?.get(i),
                third_weighing: None,
            }) {
                Ok(_) => continue,
                Err(e) => println!("Error creating grade: {:?}", e),
            }
        }
    }
    if df.clone().schema()?.index_of("calificacion1").is_some() {
        let result = df.clone()
            .lazy()
            .select(
                &[
                    col("registro"),
                    col("nombre_completo").alias("nombre_alumno"),
                    col("clave"),
                    col("nombre_duplicated_0").alias("nombre_materia"),
                    col("estatus_materia"),
                    col("nomina").alias("nomina_maestro"),
                    col("calificacion1").alias("calificacion_primer_parcial"),
                    col("faltas1").alias("faltas_primer_parcial"),
                    col("ponderacion1").alias("ponderacion_primer_parcial"),
                ]
            )
            .group_by([col("registro"), col("clave")])
            .agg(
                [
                    col("nombre_alumno").unique().first(),
                    col("nombre_materia").unique().first(),
                    col("estatus_materia").unique().first(),
                    col("nomina_maestro").unique().first(),
                    col("calificacion_primer_parcial").unique().first(),
                    col("faltas_primer_parcial").unique().first(),
                    col("ponderacion_primer_parcial").unique().first(),
                ]
            )
            .sort(["registro"], Default::default())
            .collect()?;

        for i in 0..result.height() {
            let student_id: Option<i32> = crate::schema::students::table
                .filter(
                    crate::schema::students::register.eq(result
                        .column("registro")?
                        .i32()?
                        .get(i)
                        .unwrap()),
                )
                .select(crate::schema::students::id)
                .first::<Option<i32>>(repository.conn)?;
            let teacher_id: Option<i32> = crate::schema::teachers::table
                .filter(
                    crate::schema::teachers::payroll.eq(result
                        .column("nomina_maestro")?
                        .i32()?
                        .get(i)
                        .unwrap()),
                )
                .select(crate::schema::teachers::id)
                .first::<Option<i32>>(repository.conn)?;
            let subject_id: Option<i32> = crate::schema::subjects::table
                .filter(
                    crate::schema::subjects::code.eq(result
                        .column("clave")?
                        .str()?
                        .get(i)
                        .unwrap()).and(
                        crate::schema::subjects::teacher_id.eq(teacher_id.unwrap())
                    ),
                )
                .select(crate::schema::subjects::id)
                .first::<Option<i32>>(repository.conn)?;

            match repository.create(crate::models::grade::Grade {
                id: None,
                student_id,
                subject_id,
                first_grade: result.column("calificacion_primer_parcial")?.i32()?.get(i),
                second_grade: None,
                third_grade: None,
                first_faults: result.column("faltas_primer_parcial")?.i32()?.get(i),
                second_faults: None,
                third_faults: None,
                first_weighing: result.column("ponderacion_primer_parcial")?.i32()?.get(i),
                second_weighing: None,
                third_weighing: None,
            }) {
                Ok(_) => continue,
                Err(e) => println!("Error creating grade: {:?}", e),
            }
        }
    } else {
        let result = df.clone()
            .lazy()
            .select(
                &[
                    col("registro"),
                    col("nombre_completo").alias("nombre_alumno"),
                    col("clave"),
                    col("nombre_duplicated_0").alias("nombre_materia"),
                    col("estatus_materia"),
                    col("nomina").alias("nomina_maestro"),
                ]
            )
            .group_by([col("registro"), col("clave")])
            .agg(
                [
                    col("nombre_alumno").unique().first(),
                    col("nombre_materia").unique().first(),
                    col("estatus_materia").unique().first(),
                    col("nomina_maestro").unique().first(),
                ]
            )
            .sort(["registro"], Default::default())
            .collect()?;

        for i in 0..result.height() {
            let student_id: Option<i32> = crate::schema::students::table
                .filter(
                    crate::schema::students::register.eq(result
                        .column("registro")?
                        .i32()?
                        .get(i)
                        .unwrap()),
                )
                .select(crate::schema::students::id)
                .first::<Option<i32>>(repository.conn)?;
            let teacher_id: Option<i32> = crate::schema::teachers::table
                .filter(
                    crate::schema::teachers::payroll.eq(result
                        .column("nomina_maestro")?
                        .i32()?
                        .get(i)
                        .unwrap()),
                )
                .select(crate::schema::teachers::id)
                .first::<Option<i32>>(repository.conn)?;
            let subject_id: Option<i32> = crate::schema::subjects::table
                .filter(
                    crate::schema::subjects::code.eq(result
                        .column("clave")?
                        .str()?
                        .get(i)
                        .unwrap()).and(
                        crate::schema::subjects::teacher_id.eq(teacher_id.unwrap())
                    ),
                )
                .select(crate::schema::subjects::id)
                .first::<Option<i32>>(repository.conn)?;

            match repository.create(crate::models::grade::Grade {
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
                Err(e) => println!("Error creating grade: {:?}", e),
            }
        }
    }

    Ok(())
}
