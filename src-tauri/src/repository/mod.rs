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
pub mod subject;

pub trait Repository<T> {
    fn read_all(&mut self) -> T;
    fn read(&mut self, id: i32) -> T;
    fn create(&mut self, entity: T) -> T;
    fn update(&mut self, entity: T) -> T;
    fn delete(&mut self, entity: T) -> T;
}

// pub struct StudentRepository<'a> {
//     conn: &'a mut SqliteConnection,
// }

// impl<'a> StudentRepository<'a> {
//     pub fn new(conn: &'a mut SqliteConnection) -> Self {
//         StudentRepository { conn }
//     }
// }

// impl Repository<Student> for StudentRepository<'_> {
//     fn read_all(&mut self) -> Student {
//         todo!()
//     }

//     fn read(&mut self, id: i32) -> Student {
//         todo!()
//     }

//     fn create(&mut self, entity: Student) -> Student {
//         todo!()
//     }

//     fn update(&mut self, entity: Student) -> Student {
//         todo!()
//     }

//     fn delete(&mut self, entity: Student) -> Student {
//         todo!()
//     }
// }
