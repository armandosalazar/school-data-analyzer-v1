struct StudentRepository {
    conn: &mut SqliteConnection,
}

mod TeacherRepository {
    pub fn get_teacher() {
        println!("Get teacher");
    }
}
