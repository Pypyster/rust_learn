use crate::structs::{Lesson, Student, Season, Current_Season};

mod structs;
fn main() {
    let mut student = Student{
        first_name: String::from("Petrov"),
        second_name: "Ivan".to_string(),
        age: 17,
        group: 3,
        ses: Vec::new(),
        avg_ses: 0.0,
        start_studing: 2017
    };
    student.print();
    student.add_mark(7);
    student.add_mark(3);
    student.add_mark(5);
    student.add_mark(9);
    student.add_mark(10);
    println!("Marks: {:?}", student.ses);

//    println!("{}", student.avg_ses);
    student.print();
    let years = student.studing_year(2023);
    println!("Student {} {} is starting {} years", 
        student.first_name, 
            student.second_name, 
                years);
    
    println!("________________________");

    let mut lessons = Vec::<Lesson>::new();
    lessons.push(Lesson(String::from("Math"), 2));    
    lessons.push(Lesson(String::from("IT"), 7));    
    lessons.push(Lesson(String::from("Biology"), 5));    
    lessons.push(Lesson(String::from("Philosophy"), 12));    
    lessons.push(Lesson(String::from("Art"), 6)); 

    let hours = Lesson::count_hours(&lessons);
    println!("U have {hours} hours of studing");
    let num_lessons = Lesson::count_lessons(&lessons);
    println!("U have {num_lessons} lessons");  
    let hard = Lesson::hard_lesson(&lessons);
    if let Some(lesson) = hard {
        println!("{} has most hours {}", lesson.0, lesson.1);
    } else {
        println!("No lessons");
    }
    let easy = Lesson::easy_lesson(&lessons);
    if let Some(les) = easy {        
        println!("{} has less hours {}", les.0, les.1);
    } else {
        println!("No lessons");
    }

    println!("________________________");

    let season = Current_Season::Season(Season::Winter);
    season.print();
}