@ -0,0 +1,85 @@
use std::{array, iter::Rev};

fn main(){
    let num = 2;
    let is_has_car: bool = true; 

    if num > 5 || is_has_car {
        println!("Number is bigger than 5");
    } else if num == 4{
        println!("Number is 4");
    } else {
        println!("Else operator");
    }

    // тернарник 
    let cond: bool = true;
    let number = if !cond {5} else {10};

    println!("Res: {}", number);

    // match типа switch: case
    let n1 = 7;
    match n1 {
        1 => println!(" Result 1"),
        3 => println!("Result 3"),
        _ => println!("Else"),
    }

    for i in 1..6{
        println!("Number: {}", i);
    }
    println!(" ");
      for i in 1..=6{
        println!("Number: {}", i);
    }      
    println!(" ");

      for i in (1..6).rev(){
        println!("Number: {}", i);
    } 
    println!(" ");

    for i in (1..10).rev().step_by(2){
        println!("Number: {}", i);
    }  
    println!(" ");

    let mut number1 = 4;
    while number1 > 0{
        println!("Number: {}", number1);
        number1 -= 1;
    }
    
    println!(" ");
    println!("Finished on number {}", number1); 

    for i in 1..=20{
        if i % 2 == 0 {
            continue;
        }
        if i >=9 {
            break;
        }
        println!("Number: {}", i);
    }
    println!(" ");

    //loop пока нет явного break
    let mut count: i32 = 0;
    loop {
        count += 1;
        println!("Count: {}", count);

        if count == 5 {
            break;
        }
    }
    println!(" ");
 
    let array = [10, 20, 30, 40, 52];
    for el in array{
        println!("{}", el);
    }
    
}