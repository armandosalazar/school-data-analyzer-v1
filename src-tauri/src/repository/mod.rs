use diesel::prelude::*;

use crate::schema::students;

#[derive(Debug)]
#[derive(Insertable, Queryable)]
#[diesel(table_name = students)]
pub struct Student {
    pub register: i32,
    pub fullname: String,
    pub type_: String,
    pub status: String,
    pub semester: String,
    pub group: String,
    pub turn: String,
    pub level: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl Student {
    pub fn new(
        register: i32,
        fullname: String,
        type_: String,
        status: String,
        semester: String,
        group: String,
        turn: String,
        level: String,
    ) -> Self {
        Student {
            register,
            fullname,
            type_,
            status,
            semester,
            group,
            turn,
            level,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }
}

pub trait Repository<T> {
    fn create(&mut self, entity: T) -> T;
}

pub struct StudentRepository<'a> {
    conn: &'a mut SqliteConnection,
}

impl<'a> StudentRepository<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        StudentRepository { conn }
    }
}

impl Repository<Student> for StudentRepository<'_> {
    fn create(&mut self, entity: Student) -> Student {
        let inserted_entity: Student = diesel::insert_into(students::table)
            .values(&entity)
            .get_result(self.conn)
            .expect("Error saving new student");

        println!("Student created with id: {:?}", inserted_entity);

        entity
    }
}
