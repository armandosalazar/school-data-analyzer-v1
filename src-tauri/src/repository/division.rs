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
    fn create(&mut self, entity: Division) -> Result<Division, Box<dyn std::error::Error>> {
        let division = diesel::insert_into(crate::schema::divisions::table)
            .values(&entity)
            .get_result(self.conn)?;

        Ok(division)
    }

    fn find_all(&mut self) -> Result<Vec<Division>, Box<dyn std::error::Error>> {
        todo!()
    }
}
