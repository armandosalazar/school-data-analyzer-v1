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
    fn read_all(&mut self) -> Subject {
        todo!()
    }

    fn read(&mut self, id: i32) -> Subject {
        todo!()
    }

    fn create(&mut self, entity: Subject) -> Result<Subject, Box<dyn std::error::Error>> {
        todo!()
    }

    fn update(&mut self, entity: Subject) -> Subject {
        todo!()
    }

    fn delete(&mut self, entity: Subject) -> Subject {
        todo!()
    }
}
