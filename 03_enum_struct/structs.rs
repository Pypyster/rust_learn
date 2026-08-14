#[derive(Debug)]
pub struct Student {
    pub first_name: String,
    pub second_name: String,
    pub age: u16,
    pub group: u8,
    pub ses: Vec<u8>,
    pub avg_ses: f32,
    pub start_studing: u16,

}

impl Student {
    fn avg_ses(&self) -> f32 {
        if self.ses.is_empty() {
            return 0.0;
        }

        let mut res: f32 = 0.0;
        for el in &self.ses {
            res += *el as f32;
        }
        res / (self.ses.len() as f32)
    }

    /// Добавляет оценку и сразу обновляет поле avg_ses.
    pub fn add_mark(&mut self, mark: u8) -> &mut Self {
        self.ses.push(mark);
        self.avg_ses = self.avg_ses();  

        self
    }

    pub fn studing_year(&self, year: u16) -> u8 {
        (year - self.start_studing) as u8
    }

    pub fn print(&self) -> (){
        println!("{:#?}", self)
    }
}

pub struct Lesson (pub String, pub u16);

impl Lesson {
    pub fn count_hours (lessons: &[Lesson]) -> u16 {
        lessons.iter().map(|lesson| lesson.1).sum()
    }

    pub fn count_lessons (lessons: &[Lesson]) -> u16 {
        lessons.len() as u16
    }

    pub fn hard_lesson (lessons: &[Lesson]) -> Option<&Lesson> {
        lessons.iter().max_by_key(|lesson| lesson.1)
    }

    pub fn easy_lesson (lessons: &[Lesson]) -> Option<&Lesson>{
        lessons.iter().min_by_key(|lesson| lesson.1)
    }
    
}

pub enum Season{
    Winter,
    Autumn,
    Spring,
    Summer,
}

pub enum Current_Season {
    Season(Season),
}

impl Current_Season {
    pub fn print(&self) {
        match self {
            Current_Season::Season(Season::Autumn) => println!("Now is autumn!"),
            Current_Season::Season(Season::Spring) => println!("Now is spring!"),
            Current_Season::Season(Season::Summer) => println!("Now is summer!"),
            Current_Season::Season(Season::Winter) => println!("Now is winter!"),
        }
    }
}