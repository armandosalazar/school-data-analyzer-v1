use diesel::prelude::*;

#[derive(Insertable, Queryable, Identifiable, Associations, PartialEq)]
#[derive(Debug)]
#[diesel(belongs_to(crate::models::student::Student))]
#[diesel(belongs_to(crate::models::subject::Subject))]
#[diesel(table_name = crate::schema::grades)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Grade {
    pub id: Option<i32>,
    pub student_id: Option<i32>,
    pub subject_id: Option<i32>,
    pub first_grade: Option<i32>,
    pub second_grade: Option<i32>,
    pub third_grade: Option<i32>,
    pub first_faults: Option<i32>,
    pub second_faults: Option<i32>,
    pub third_faults: Option<i32>,
    pub first_weighing: Option<i32>,
    pub second_weighing: Option<i32>,
    pub third_weighing: Option<i32>,
}

impl Grade {
    pub fn new(
        student_id: i32,
        subject_id: i32,
        first_grade: i32,
        second_grade: i32,
        third_grade: i32,
        first_faults: i32,
        second_faults: i32,
        third_faults: i32,
        first_weighing: i32,
        second_weighing: i32,
        third_weighing: i32,
    ) -> Self {
        Grade {
            id: None,
            student_id: Some(student_id),
            subject_id: Some(subject_id),
            first_grade: Some(first_grade),
            second_grade: Some(second_grade),
            third_grade: Some(third_grade),
            first_faults: Some(first_faults),
            second_faults: Some(second_faults),
            third_faults: Some(third_faults),
            first_weighing: Some(first_weighing),
            second_weighing: Some(second_weighing),
            third_weighing: Some(third_weighing),
        }
    }
}
