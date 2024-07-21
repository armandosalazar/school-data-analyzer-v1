use diesel::prelude::*;

use crate::models::subject::Subject;

use super::Repository;

pub struct SubjectRepository<'a> {
    pub conn: &'a mut SqliteConnection,
}

impl<'a> SubjectRepository<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        SubjectRepository { conn }
    }
}

impl Repository<Subject> for SubjectRepository<'_> {
    fn create(&mut self, entity: Subject) -> Result<Subject, Box<dyn std::error::Error>> {
        let subject = diesel::insert_into(crate::schema::subjects::table)
            .values(&entity)
            .get_result(self.conn)?;

        Ok(subject)
    }

    fn find_all(&mut self) -> Result<Vec<Subject>, Box<dyn std::error::Error>> {
        todo!()
    }
}
