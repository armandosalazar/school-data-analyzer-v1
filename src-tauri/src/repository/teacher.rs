use diesel::prelude::*;

use crate::models::teacher::Teacher;

use super::Repository;

pub struct TeacherRepository<'a> {
    pub conn: &'a mut SqliteConnection,
}

impl<'a> TeacherRepository<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        TeacherRepository { conn }
    }
}

impl Repository<Teacher> for TeacherRepository<'_> {
    fn read_all(&mut self) -> Teacher {
        todo!()
    }

    fn read(&mut self, id: i32) -> Teacher {
        todo!()
    }

    fn create(&mut self, entity: Teacher) -> Result<Teacher, Box<dyn std::error::Error>> {
        let row = diesel::insert_into(crate::schema::teachers::table)
            .values(&entity)
            .get_result(self.conn)?;

        Ok(row)
    }

    fn update(&mut self, entity: Teacher) -> Teacher {
        todo!()
    }

    fn delete(&mut self, entity: Teacher) -> Teacher {
        todo!()
    }
}
