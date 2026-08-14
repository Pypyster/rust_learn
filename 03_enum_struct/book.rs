#[derive(Debug)] // выводить объекты напрямую
pub struct Book {
    pub title: String,
    pub aurth: String,
    pub year: u32,
    pub pages: u32,
}

impl Book{
    pub fn new(title: &str, aurth: &str, year: u32, pages:  u32) -> Self {
        Self { 
            title: title.to_string(), 
            aurth: aurth.to_string(),
            year,
            pages
        }
    }
}