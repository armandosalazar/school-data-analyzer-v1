use diesel::prelude::*;

use super::Repository;
use crate::models::teacher::Teacher;

pub struct TeacherRepository<'a> {
    pub conn: &'a mut SqliteConnection,
}

impl<'a> TeacherRepository<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        TeacherRepository { conn }
    }
}

impl Repository<Teacher> for TeacherRepository<'_> {
    fn find_all(&mut self) -> Result<Vec<Teacher>, Box<dyn std::error::Error>> {
        let teachers = crate::schema::teachers::table.load::<Teacher>(self.conn)?;

        Ok(teachers)
    }

    fn create(&mut self, entity: Teacher) -> Result<Teacher, Box<dyn std::error::Error>> {
        let teacher = diesel::insert_into(crate::schema::teachers::table)
            .values(&entity)
            .get_result(self.conn)?;

        Ok(teacher)
    }
}
