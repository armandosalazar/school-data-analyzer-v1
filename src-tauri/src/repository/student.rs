use std::error::Error;
use diesel::prelude::*;

use crate::repository::Repository;
use crate::models::student::Student;

pub struct StudentRepository<'a> {
    pub conn: &'a mut SqliteConnection,
}

impl<'a> StudentRepository<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        StudentRepository { conn }
    }
}

impl Repository<Student> for StudentRepository<'_> {
    fn count(&mut self) -> Result<i64, Box<dyn Error>> {
        todo!()
    }

    fn create(&mut self, entity: Student) -> Result<Student, Box<dyn Error>> {
        let student = diesel::insert_into(crate::schema::students::table)
            .values(&entity)
            .get_result(self.conn)?;

        Ok(student)
    }

    fn find_all(&mut self, offset: Option<i64>, page_size: Option<i64>, sort_field: Option<String>, sort_order: Option<i64>, filters: Option<String>) -> Result<Vec<Student>, Box<dyn Error>> {
        todo!()
    }
}