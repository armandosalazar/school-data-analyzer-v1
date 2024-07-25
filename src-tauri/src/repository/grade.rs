use std::error::Error;
use diesel::prelude::*;

use super::Repository;
use crate::models::grade::Grade;

pub struct GradeRepository<'a> {
    pub conn: &'a mut SqliteConnection,
}

impl<'a> GradeRepository<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        GradeRepository { conn }
    }
}

impl Repository<Grade> for GradeRepository<'_> {
    fn count(&mut self) -> Result<i64, Box<dyn Error>> {
        todo!()
    }

    fn create(&mut self, entity: Grade) -> Result<Grade, Box<dyn Error>> {
        let grade = diesel::insert_into(crate::schema::grades::table)
            .values(&entity)
            .get_result(self.conn)?;

        Ok(grade)
    }

    fn find_all(&mut self, offset: Option<i64>, page_size: Option<i64>, filters: Option<String>) -> Result<Vec<Grade>, Box<dyn Error>> {
        todo!()
    }
}
