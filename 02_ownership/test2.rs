macro_rules! print_my {
    ($($arg:tt)*) => {
        println!($($arg)*);
    };
}

fn length_owner_string (s: String) -> u16 {
    s.len() as u16
}

fn length_unowner_string (s: &String) -> u16 {
    s.len() as u16
}

fn length_str(s: &str) -> u16{
     s.chars().count().try_into().unwrap()
}

fn first_num (s: &String) -> Option<u8> {
    s.chars()
        .next()
            .and_then(|c| c.to_digit(10))
                .map(|d| d as u8)
}
fn main() {
    let  str1: String = "Hi".to_string();

    let l1 = length_unowner_string(&str1);
    println!("The word {str1} has length {l1}");

    let l2 = length_owner_string(str1.clone());// передаем клона чтоб не потерять владение
    println!("The word {str1} has length {l2}");

    println!("__________________________");

    let str2: &str = "Rust learning"; // заимствуем 
    let l3 = length_str(str2);
    println!("{str2} length is {l3}");

    println!("__________________________");
    
    let arr = ["123".to_string(), "456".to_string(), "789".to_string(), "A74".to_string()];

    for el in &arr{
        if let Some(d) = first_num(&el) {
            println!("First num is {d}");
        } else {
            print_my!("First char isn't a number");
        }
    }
    println!("{:?}", arr); // можем вывести т.к. в цикле передаем ссылку


}