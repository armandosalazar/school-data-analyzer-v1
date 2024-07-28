use diesel::prelude::*;

use super::Repository;
use crate::models::teacher::Teacher;

pub struct TeacherRepository<'a> {
    pub conn: &'a mut SqliteConnection,
}

impl<'a> TeacherRepository<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        TeacherRepository { conn }
    }
}

impl Repository<Teacher> for TeacherRepository<'_> {
    fn count(&mut self) -> Result<i64, Box<dyn std::error::Error>> {
        let count = crate::schema::teachers::table
            .count()
            .get_result::<i64>(self.conn)?;

        Ok(count)
    }

    fn create(&mut self, entity: Teacher) -> Result<Teacher, Box<dyn std::error::Error>> {
        let teacher = diesel::insert_into(crate::schema::teachers::table)
            .values(&entity)
            .get_result(self.conn)?;

        Ok(teacher)
    }

    fn find_all(
        &mut self,
        offset: Option<i64>,
        page_size: Option<i64>,
        sort_field: Option<String>,
        sort_order: Option<i64>,
        filters: Option<String>,
    ) -> Result<Vec<Teacher>, Box<dyn std::error::Error>> {

        let teacher_filters: crate::models::teacher::TeacherFilters =
            serde_json::from_str::<crate::models::teacher::TeacherFilters>(
                filters.unwrap().as_str(),
            )?;

        println!("Filters: {:?}", teacher_filters);

        let mut query = crate::schema::teachers::table.into_boxed();

        if let Some(filter) = teacher_filters.id {
            if let Some(value) = filter.value {
                match filter.match_mode.as_str() {
                    "startsWith" => {
                        query = query.filter(crate::schema::teachers::id.eq(value.parse::<i32>()?))
                    }
                    _ => {}
                }
            }
        }
        if let Some(filter) = teacher_filters.name {
            if let Some(value) = filter.value {
                match filter.match_mode.as_str() {
                    "startsWith" => {
                        query =
                            query.filter(crate::schema::teachers::name.like(format!("{}%", value)))
                    }
                    "contains" => {
                        query =
                            query.filter(crate::schema::teachers::name.like(format!("%{}%", value)))
                    }
                    "notContains" => {
                        query = query.filter(
                            crate::schema::teachers::name
                                .not_like(format!("%{}%", value))
                                .or(crate::schema::teachers::name.not_like(format!("{}%", value))),
                        )
                    }
                    "endsWith" => {
                        query =
                            query.filter(crate::schema::teachers::name.like(format!("%{}", value)))
                    }
                    "equals" => query = query.filter(crate::schema::teachers::name.eq(value)),
                    "notEquals" => query = query.filter(crate::schema::teachers::name.ne(value)),
                    _ => {}
                }
            }
        }
        if let Some(filter) = teacher_filters.payroll {
            if let Some(value) = filter.value {
                match filter.match_mode.as_str() {
                    "startsWith" => {
                        query =
                            query.filter(crate::schema::teachers::payroll.eq(value.parse::<i32>()?))
                    }
                    _ => {}
                }
            }
        }

        // match sort_field.as_deref() {
        //     Some("id") => query = match sort_order.unwrap() {
        //         1 => query.order(crate::schema::teachers::id.asc()),
        //         -1 => query.order(crate::schema::teachers::id.desc()),
        //         _ => query,
        //     },
        //     Some("name") => query = match sort_order.unwrap() {
        //         1 => query.order(crate::schema::teachers::name.asc()),
        //         -1 => query.order(crate::schema::teachers::name.desc()),
        //         _ => query,
        //     },
        //     Some("payroll") => query = match sort_order.unwrap() {
        //         1 => query.order(crate::schema::teachers::payroll.asc()),
        //         -1 => query.order(crate::schema::teachers::payroll.desc()),
        //         _ => query,
        //     },
        //     _ => {}
        // };

        let teachers = query
            .limit(page_size.unwrap())
            .offset(offset.unwrap())
            .load::<Teacher>(self.conn)?;

        Ok(teachers)
    }
}
