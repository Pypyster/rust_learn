use std::io;

fn main() {
    let num1: i32 = 154;
    let num2: i64 = 74;
    let res: i64 = num2 + num1 as i64;

    let txt: &str = "Привет";
    let blength = txt.len();
    let length = txt.chars().count();
    println!("Bytes: {}, chars: {}, sum: {}", blength, length, res);

    if let Some(first_char) = txt.chars().next() {
        println!("First symbol: {}", first_char);
    } else {
        println!("Empty!");
    }

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Reading failed");
    let input_num: i32 = input.trim().parse().expect("Not a number");

    if input_num < 10 {
        println!("The number {} is less than 10", input_num);
    } else if input_num % 2 == 0 {
        println!("The number {} is even", input_num);
    } else {
        println!("The number {} is odd", input_num);
    }

    let mut divisors: Vec<i32> = Vec::new();
    let limit = (input_num as f64).sqrt() as i32;

    for i in 1..=limit {
        if input_num % i == 0 {
            divisors.push(i);
            if i != input_num / i {
                divisors.push(input_num / i);
            }
        }
    }

    divisors.sort();
    println!("Divisors: {:?}", divisors);

    let mut pet: (&str, u8, bool) = ("Merphy", 9, false);
    let mut count = 0;

    loop {
        pet.1 += 1;
        count +=1;

        if pet.1 == 17 {
            println!("After {} years ago, pet will be {}", count, pet.1);
            break;
        }
    }

    let commands = ["start", "stop", "pause", "sit", "unknown", "exit", "start"];

    for cmd in commands{
        match cmd {
            "start" => println!("Engine start"),
            "stop" => println!("Engine stop"),
            "pause" => println!("Engine pause"),
            "exit" =>{
                println!("Exit from car");
                break;
            } 
            _ => println!("Unknown action")
        }
    }


}