use diesel::prelude::*;

#[derive(Insertable, Queryable, Identifiable)]
#[diesel(table_name = crate::schema::divisions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Division {
    id: Option<i32>,
    code: Option<i32>,
    academy: Option<String>,
}

impl Division {
    pub fn new(code: i32, academy: String) -> Self {
        Division {
            id: None,
            code: Some(code),
            academy: Some(academy),
        }
    }
}
