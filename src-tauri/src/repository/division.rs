use diesel::prelude::*;

use super::Repository;
use crate::models::division::Division;

pub struct DivisionRepository<'a> {
    pub conn: &'a mut SqliteConnection,
}

impl<'a> DivisionRepository<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        DivisionRepository { conn }
    }
}

impl Repository<Division> for DivisionRepository<'_> {
    fn find_all(
        &mut self,
        offset: Option<i64>,
        page_size: Option<i64>,
    ) -> Result<Vec<Division>, Box<dyn std::error::Error>> {
        todo!()
    }

    fn create(&mut self, entity: Division) -> Result<Division, Box<dyn std::error::Error>> {
        let division = diesel::insert_into(crate::schema::divisions::table)
            .values(&entity)
            .get_result(self.conn)?;

        Ok(division)
    }

    fn count(&mut self) -> Result<i64, Box<dyn std::error::Error>> {
        todo!()
    }
}
