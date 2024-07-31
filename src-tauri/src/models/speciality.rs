use diesel::prelude::*;
use serde::Serialize;

#[derive(Insertable, Queryable)]
#[derive(Debug)]
#[diesel(table_name = crate::schema::specialities)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(Serialize)]
pub struct Speciality {
    pub id: Option<i32>,
    pub code: Option<i32>,
    pub name: Option<String>,
}

impl Speciality {
    pub fn new(code: i32, name: String) -> Self {
        Speciality {
            id: None,
            code: Some(code),
            name: Some(name),
        }
    }
}
