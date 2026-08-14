use std::fs::File;
use std::io::{self, BufRead};
use std::mem::replace;
use std::vec;

fn read_file_lines(path: &str) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line?);
    }

    Ok(lines)
}

fn slice_chars(s: &str, start: usize, end: usize) -> String {
    s.chars()
        .skip(start)
        .take(end - start)
        .collect()
}

fn char_arr(s: &str) -> Vec<char>{
    let mut res: Vec<char> = Vec::new();
    for (i,c) in s.chars().enumerate(){
        if i % 3 == 0 {
            res.push(c);
        }
    }
    res
}

fn split_space (s: String) -> Vec<(usize, String)> {
    let mut res: Vec<(usize, String)> = Vec::new();

    for (i,word) in s.split_whitespace().enumerate(){
        res.push((i, word.to_owned()));
    }
    res
}

fn split_char(s: &str, c: char) -> Vec<String> {
    s.split(c)
        .map(str::to_string)  
        .collect()             // собираем в Vec<String>
}


fn main() -> std::io::Result<()> {
    let lines = read_file_lines("text.txt")?;
    //println!("{:?}", lines);
    
    let mut s1 = lines[0].clone();

    println!("{s1}");

    println!("___________________________");

    let  s2 : &str = &s1[2..6]; // байты
    println!("{s2}");
    let mut s3: String = slice_chars(&s1, 0, 7); 
    println!("{s3}");

    println!("___________________________");

    let arr = char_arr(&s1);
    for (i, el) in arr.iter().enumerate() {
        println!("{} element is '{}'", i*3, el);
    }

    println!("___________________________");

    let mut s4 = String::from ("Hello");
    s4.push_str(" world");
    s4.push('!');
    println!("{s4}");
    
    println!("___________________________");

    let mut s5 = lines[5].clone();
    println!("{s5}");

    let vec_res = split_space(s5);
    println!("Words in text : {:?}", vec_res);

    println!("___________________________");

    let s6 = lines[18].clone();
    println!("{s6}");
    let word_arr = split_char(&s6, 'о');
    println!("{:?}", word_arr);

    println!("___________________________");

    let mut s7 = lines[8].clone();
    println!("{s7}");
     s7 = s7.replace("add", "Hi");
    println!("{s7}");
    s7.replace_range(7..16, " new string ");
    println!("{s7}");

    Ok(())
}