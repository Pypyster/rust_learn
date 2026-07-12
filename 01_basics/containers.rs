use std::collections::HashMap;
fn main(){
//  vector
    let mut v: Vec<i32> = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);
    v.push(40);
    v.push(52);
    v.push(67);

    println!("Vec: {:?}", v); // :? чтоб вывести все элементы 

    v[0] = 4964;
    println!("El 1: {}", v[0]);

    let mut v2 = vec![1,2,5,6,8,1,4,54,84];
    v2.push(57);
    println!("Vector_2: {:?}", v2);

    match v2.get(12){
        Some(value) => println!("El in v2: {}", value),
        None => println!("Error no element"),// если нет такого индекса 
    }

    let mut v3 = vec![100, 202, 301];
    v3.pop(); // удаляет последний
    v3.push(565);
    v3.remove(1);
    for value in &v3{
        println!("El: {value}");
    } 

    // string ass array of char 
    let  s1 = String :: new();
    let  s2 = String :: from("Hello Rust");

    println!("{s2}");

    let s3 = s1 + &s2;

    let mut word  = String :: new();
    word.push_str("Hi");
    word.push(' ');
    word.push_str("Merphy");
    println!("{word}");

    //хэш-мап
    let mut scores  = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Red", 5);
    scores.insert("White", 31);
    scores.insert("Black", -10);
    scores.insert("Green", 0);

    scores.remove("White");

    println!("HasMap: {:?}", scores);

    scores.insert("Red", 15);
    println!("Hash: {}", scores.get("Red").unwrap());
    

}