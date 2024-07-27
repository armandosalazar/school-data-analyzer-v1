use std::error::Error;
use diesel::prelude::*;

use super::Repository;
use crate::models::speciality::Speciality;

pub struct SpecialityRepository<'a> {
    pub conn: &'a mut SqliteConnection,
}

impl<'a> SpecialityRepository<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        SpecialityRepository { conn }
    }
}

impl Repository<Speciality> for SpecialityRepository<'_> {
    fn count(&mut self) -> Result<i64, Box<dyn Error>> {
        todo!()
    }

    fn create(&mut self, entity: Speciality) -> Result<Speciality, Box<dyn Error>> {
        let speciality = diesel::insert_into(crate::schema::specialities::table)
            .values(&entity)
            .get_result(self.conn)?;

        Ok(speciality)
    }

    fn find_all(&mut self, offset: Option<i64>, page_size: Option<i64>, sort_field: Option<String>, sort_order: Option<i64>, filters: Option<String>) -> Result<Vec<Speciality>, Box<dyn Error>> {
        todo!()
    }
}
