use diesel::prelude::*;

#[derive(Insertable, Queryable)]
#[diesel(table_name = crate::schema::teachers)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Teacher {
    id: Option<i32>,
    payfoll: Option<i32>,
    name: Option<String>,
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
