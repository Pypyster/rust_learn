use std::{clone, range};
use std::collections::HashSet;
pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}

fn is_pangram(s: &str) -> bool {
    let mut letters = HashSet::new(); // проверяет есть\нет 
    for c in s.chars() {
        if c.is_ascii_alphabetic(){
            letters.insert(c.to_ascii_lowercase());
        }
    }
    letters.len() == 26
}

fn is_triangle(a: i64, b: i64, c: i64) -> bool {
    let sum = a + b + c;
    if a <= 0 || b<=0 || c<=0 {
        false
    } else  {
        a.max(b).max(c) < sum - a.max(b).max(c)
    }
}

fn get_middle(s: &str) -> &str {
    let even = s.len() % 2 == 0;
    let middle = s.len() / 2;

    &s[if even { middle - 1 } else { middle }..middle + 1]
}

fn square_digits(num: u64) -> u64 {
    let s = num.to_string();
    let square: String = s.chars()
    .map(|c|{
        let d = c.to_digit(10).unwrap();
        (d*d).to_string()})
        .collect();
    square.parse::<u64>().unwrap()
}

use std::iter::FromIterator;

fn unique_in_order<T>(iter: T) -> Vec<T::Item>
where
    T: IntoIterator,
    T::Item: PartialEq,
{
    let mut vec = Vec::from_iter(iter);
    vec.dedup();
    vec
}

fn comp(  a: Vec<i64>, b: Vec<i64>) -> bool {
    if a.len() != b.len(){
        return false;
    } else if a.is_empty() || b.is_empty() {
        return false;
    }
    let mut a_sq: Vec<i64> = a.iter().map(|x| x*x).collect();
    let mut b_sort : Vec<i64> = b.clone();
    a_sq.sort();
    b_sort.sort();
    a_sq == b_sort
}

fn rot13(message: &str) -> String {
    let mut res = String::new();
    for c in message.chars() {
        if c.is_ascii_lowercase() {
            let base = b'a';
            let offset = (c as u8 - base + 13) % 26;
            res.push((base + offset) as char);

        } else if c.is_ascii_uppercase() {
            let base = b'A';
            let offset = (c as u8 - base + 13) % 26;
            res.push((base + offset) as char);
        } else {
            res.push(c);
        }
    }    
    res
}

fn is_valid_walk(walk: &[char]) -> bool {
    if walk.len() != 10 {
        return false;
    }
    let mut ns = 0;
    let mut we = 0;
    for &c in walk {
        match  c {
            'n' => ns+= 1,
            's' => ns-= 1,
            'w' => we+= 1,
            'e' => we-= 1,
            _ => return false
        }
    }
    ns == 0 && we == 0
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    North,
    East,
    West,
    South,
}

impl Direction {
    fn opposite(&self, other: &Direction) -> bool {
        match self {
            Direction::North => *other == Direction::South,
            Direction::East => *other == Direction::West,
            Direction::West => *other == Direction::East,
            Direction::South => *other == Direction::North
        }
    }
}

fn dir_reduc(arr: &[Direction]) -> Vec<Direction> {
    let mut res : Vec<Direction> = vec![];
    for d in arr {
        if !res.is_empty() && res.last().unwrap().opposite(d) {
            res.pop();
        } else {
            res.push(*d);
        }
    }
    res
}
use Direction::{East, North, South, West};

fn find_uniq(arr: &[f64]) -> f64 {
    let mut vec_num = arr.to_vec();
    vec_num.sort_by(|a, b| a.partial_cmp(b).unwrap()); // аккуратнее с f64
    if vec_num[0] == vec_num[1]{
        *vec_num.last().unwrap()
    } else {
        vec_num[0]
    }
}

fn main() {
    let input = "Stress";
    println!("{}", reverse(&input));

    let s = "ABCD45EFGH,IJK,LMNOPQR56STUVW3XYZ";
    println!("{}", is_pangram(&s));
    println!("__________________________________________");

    println!("{}", is_triangle(1, 2, 2));
    println!("{}", is_triangle(7, 2, 2));
    println!("{}", is_triangle(1, 2, 0));

    println!("__________________________________________");
    println!("{}", get_middle("middle"));
    println!("{}", get_middle("A"));
    println!("{}", get_middle("testing"));

    println!("__________________________________________");
    println!("{}", square_digits(9112));

    println!("__________________________________________");
    println!("{:?}", unique_in_order([1,2,2,3,3,4,4,5,5,5,6,6]));


    println!("__________________________________________");
    let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
    let a2 = vec![11*11, 121*121, 144*144, 19*19, 161*161, 19*19, 144*144, 19*19];
    println!("{}", comp(a1,a2));
    let a1 = vec![10000001, 100000000];
    let a2 = vec![10000000 * 10000000, 100000000 * 100000000];
    println!("{}", comp(a1, a2.clone()));
    println!("{}", comp(vec![], vec![]));

    println!("__________________________________________");
    println!("{}", rot13("test"));

    println!("__________________________________________");
    println!("{}", is_valid_walk(&['w','e','w','e','w','e','w','e','w','e','w','e']));
    println!("{}", is_valid_walk(&['n','s','n','s','n','s','n','s','n','s']));

    println!("__________________________________________");
    println!("{:#?}", dir_reduc(&[North, South, South, East, West, North, West]));

    println!("__________________________________________");
    println!("{}", find_uniq(&[1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.01,1.0,1.0,1.0,1.0]));

}