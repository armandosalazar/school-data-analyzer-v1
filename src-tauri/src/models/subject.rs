use diesel::prelude::*;

use crate::models::division::Division;
use crate::models::teacher::Teacher;

#[derive(Insertable, Queryable, Associations)]
#[diesel(belongs_to(Teacher))]
#[diesel(belongs_to(Division))]
#[diesel(table_name = crate::schema::subjects)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Subject {
    id: Option<i32>,
    code: Option<String>,
    name: Option<String>,
    teacher_id: Option<i32>,
    division_id: Option<i32>,
}
