use diesel::prelude::*;
use serde::Serialize;
// use crate::schema::students::speciality_id;

#[derive(Insertable, Queryable, Debug, Identifiable)]
#[diesel(table_name = crate::schema::students)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(Serialize)]
pub struct Student {
    pub id: Option<i32>,
    pub speciality_id: Option<i32>,
    pub register: Option<i32>,
    pub name: Option<String>,
    pub type_: Option<String>,
    pub status: Option<String>,
    pub semester: Option<i32>,
    pub group: Option<String>,
    pub turn: Option<String>,
    pub level: Option<String>,
}

impl Student {
    pub fn new(
        speciality_id: i32,
        register: i32,
        name: String,
        type_: String,
        status: String,
        semester: i32,
        group: String,
        turn: String,
        level: String,
    ) -> Self {
        Student {
            id: None,
            speciality_id: Some(speciality_id),
            register: Some(register),
            name: Some(name),
            type_: Some(type_),
            status: Some(status),
            semester: Some(semester),
            group: Some(group),
            turn: Some(turn),
            level: Some(level),
        }
    }
}
