pub struct Speciality {
    pub id: Option<i32>,
    pub code: Option<i32>,
    pub name: Option<String>,
}

impl Speciality {
    pub fn new(code: i32, name: String) -> Self {
        Speciality {
            id: None,
            code: Some(code),
            name: Some(name),
        }
    }
}
