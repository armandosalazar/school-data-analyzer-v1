pub mod division;
pub mod subject;
pub mod teacher;

pub trait Repository<T> {
    // fn read_all(&mut self) -> T;
    // fn read(&mut self, id: i32) -> T;
    fn create(&mut self, entity: T) -> Result<T, Box<dyn std::error::Error>>;
    // fn update(&mut self, entity: T) -> T;
    // fn delete(&mut self, entity: T) -> T;
}
