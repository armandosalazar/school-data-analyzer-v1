use diesel::prelude::*;

use super::Repository;
use crate::models::speciality::Speciality; wh

pub struct SpecialityRepository<'a> {
    pub conn: &'a mut SqliteConnection,
}

impl<'a> SpecialityRepository<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        SpecialityRepository { conn }
    }
}

impl Repository<Speciality> for Speci
