use rand::RngExt;
mod book;
use book::Book; // сразу обращение к структуре
use inquire::Text;

fn main() {
    /*let mut rng = rand::rng();           
    let rng_num: u32 = rng.random_range(1..=101); 
    println!("Random number is {}", rng_num);*/

    let mut library = Vec::new(); 

    loop {
        let title = Text::new("Enter the title").prompt().unwrap();
        let aurth = Text::new("Enter the author").prompt().unwrap();
        let year = Text::new("Enter the year").prompt().unwrap().parse::<u32>().unwrap();
        
        let mut rng = rand::rng();           
        let pages: u32 = rng.random_range(1000..=10000);

        library.push(Book::new(&title, &aurth, year, pages));

        println!("Current library:");
        for book in &library{
            println!("{:?}", book);
        }

        let mut change = Text::new("Add another book? yes/no").prompt().unwrap();

        match change.trim().to_lowercase().as_str() {
            "yes" => continue,
            _ => break,
        }

    }
}