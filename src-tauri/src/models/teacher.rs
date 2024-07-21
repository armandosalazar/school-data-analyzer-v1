use diesel::prelude::*;
use serde::Serialize;

#[derive(Insertable, Queryable, Identifiable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::teachers)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Teacher {
    pub id: Option<i32>,
    pub payfoll: Option<i32>,
    pub name: Option<String>,
}

impl Teacher {
    pub fn new(payfoll: i32, name: String) -> Self {
        Teacher {
            id: None,
            payfoll: Some(payfoll),
            name: Some(name),
        }
    }
}
