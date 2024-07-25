pub mod division;
pub mod subject;
pub mod teacher;
pub mod speciality;
pub mod student;
pub mod grade;

pub trait Repository<T> {
    fn count(&mut self) -> Result<i64, Box<dyn std::error::Error>>;
    fn create(&mut self, entity: T) -> Result<T, Box<dyn std::error::Error>>;
    fn find_all(
        &mut self,
        offset: Option<i64>,
        page_size: Option<i64>,
        filters: Option<String>,
    ) -> Result<Vec<T>, Box<dyn std::error::Error>>;
    // fn read_all(&mut self) -> T;
    // fn read(&mut self, id: i32) -> T;
    // fn update(&mut self, entity: T) -> T;
    // fn delete(&mut self, entity: T) -> T;
}
