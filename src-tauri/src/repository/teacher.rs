use chrono::offset;
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
    fn find_all(
        &mut self,
        offset: Option<i64>,
        page_size: Option<i64>,
    ) -> Result<Vec<Teacher>, Box<dyn std::error::Error>> {
        let teachers = crate::schema::teachers::table
            .limit(page_size.unwrap())
            .offset(offset.unwrap())
            .load::<Teacher>(self.conn)?;

        Ok(teachers)
    }

    fn create(&mut self, entity: Teacher) -> Result<Teacher, Box<dyn std::error::Error>> {
        let teacher = diesel::insert_into(crate::schema::teachers::table)
            .values(&entity)
            .get_result(self.conn)?;

        Ok(teacher)
    }

    fn count(&mut self) -> Result<i64, Box<dyn std::error::Error>> {
        let count = crate::schema::teachers::table
            .count()
            .get_result::<i64>(self.conn)?;

        Ok(count)
    }
}
