use std::convert::{AsMut,AsRef};
use std::ops::Mul;

use reqwest::Error;

fn count_symbol<T:AsRef<str>> (s: T) -> usize {
    s.as_ref().chars().count()
}

fn num_sq<T>(x: T) -> T
where
    T: Copy + Mul<Output = T>,
{
    x * x
}

fn extension<'a> (input: &'a str) -> Option<&'a str> {
    input.rsplit_once('.').map(|(_,ext)| ext)
}

fn strlen(mut ptr: *const u8) -> usize {
    let mut len = 0;
    unsafe{
        while *ptr !=0 {
            len+= 1;
            ptr =ptr.add(1);
        }
    }
    len
}

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}
#[derive(Debug)]

enum ColorErr {
    BadInput(String),
}

fn get_color(input: &str) -> Result<Color, ColorErr> {
    let mut parts = input.split(',');

    let red = parts
        .next()
        .ok_or(ColorErr::BadInput("missing red".to_string()))?
        .parse::<u8>()
        .map_err(|_| ColorErr::BadInput("invalid red".to_string()))?;

    let green = parts
        .next()
        .ok_or(ColorErr::BadInput("missing green".to_string()))?
        .parse::<u8>()
        .map_err(|_| ColorErr::BadInput("invalid green".to_string()))?;

    let blue = parts
        .next()
        .ok_or(ColorErr::BadInput("missing blue".to_string()))?
        .parse::<u8>()
        .map_err(|_| ColorErr::BadInput("invalid blue".to_string()))?;

    Ok(Color { red, green, blue })
}
fn main() {
    let s1 = String::from("Hello, Rust!");
    println!("Length of string: {}", count_symbol(s1));

    let s2 = "New string";
    println!("Length of string_2: {}", count_symbol(s2));

    let s3: &String = &"The last string".to_string();
    println!("Length of last string: {}", count_symbol(s3));

    let num1 = num_sq(4.12);
    println!("Square of number: {num1}");

    let num2 = num_sq(3);
    println!("Square of number: {num2}");

    let s = String::from("file.txt");
    if let Some(exp) = extension(&s){
        println!("{exp}");
    }
    let s = "暂停使用\0";
    println!("{} has {} bytes", s, strlen(s.as_ptr()));

    match get_color("255,128,0") {
    Ok(color) => println!("{} {} {}", color.red, color.green, color.blue),
    Err(e) => println!("error: {:?}", e),
    }

    match get_color(",128,0") {
    Ok(color) => println!("{} {} {}", color.red, color.green, color.blue),
    Err(e) => println!("error: {:?}", e),
    }
    match get_color("255,444,0") {
    Ok(color) => println!("{} {} {}", color.red, color.green, color.blue),
    Err(e) => println!("error: {:?}", e),
    }

}
