use diesel::internal::derives::multiconnection::SelectStatementAccessor;
use diesel::prelude::*;
use diesel::query_dsl::methods::GroupByDsl;

#[tauri::command]
pub fn get_students() {
    let mut conn = crate::database::establish_connection();

    let result = crate::schema::students::dsl::students
        .limit(10)
        .load(&mut conn)
        .expect("Error loading students");

    let grades = crate::models::grade::Grade::belonging_to(&result)
        .load::<crate::models::grade::Grade>(&mut conn)
        .expect("Error loading grades");

    println!("{:?}", grades);

    println!("{:?}", result);
}
