use crate::repository::Repository;

use diesel::prelude::*;

use crate::models::subject::Subject;

pub struct SubjectRepository<'a> {
    pub conn: &'a mut SqliteConnection,
}

impl<'a> SubjectRepository<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        SubjectRepository { conn }
    }
}

impl<'a> Repository<Subject<'a>> for SubjectRepository<'a> {
    fn read_all(&mut self) -> Subject<'a> {
        todo!()
    }

    fn read(&mut self, id: i32) -> Subject<'a> {
        todo!()
    }

    fn create(&mut self, entity: Subject<'a>) -> Subject<'a> {
        entity
    }

    fn update(&mut self, entity: Subject<'a>) -> Subject<'a> {
        todo!()
    }

    fn delete(&mut self, entity: Subject<'a>) -> Subject<'a> {
        todo!()
    }
}
