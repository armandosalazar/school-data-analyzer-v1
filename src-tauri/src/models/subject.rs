pub struct Subject {
    name: String,
    code: String,
}

impl Subject {
    pub fn new(name: String, code: String) -> Self {
        Subject { name, code }
    }
}
