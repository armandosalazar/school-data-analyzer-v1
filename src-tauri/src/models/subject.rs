pub struct Subject<'a> {
    id: &'a Option<u32>,
    name: &'a str,
    code: &'a str,
}

impl<'a> Subject<'a> {
    pub fn new(name: &'a str, code: &'a str) -> Self {
        Subject {
            id: &None,
            name,
            code,
        }
    }
}
