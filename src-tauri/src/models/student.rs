// #[derive(Debug, Insertable, Queryable)]
// #[diesel(table_name = students)]
// pub struct Student {
//     pub register: i32,
//     pub fullname: String,
//     pub type_: String,
//     pub status: String,
//     pub semester: String,
//     pub group: String,
//     pub turn: String,
//     pub level: String,
//     pub created_at: chrono::NaiveDateTime,
//     pub updated_at: chrono::NaiveDateTime,
// }

// impl Student {
//     pub fn new(
//         register: i32,
//         fullname: String,
//         type_: String,
//         status: String,
//         semester: String,
//         group: String,
//         turn: String,
//         level: String,
//     ) -> Self {
//         Student {
//             register,
//             fullname,
//             type_,
//             status,
//             semester,
//             group,
//             turn,
//             level,
//             created_at: chrono::Local::now().naive_local(),
//             updated_at: chrono::Local::now().naive_local(),
//         }
//     }
// }
