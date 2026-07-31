// Box аллоцировать что-то в куче, автоматически очищается 
//  есть тип с неизвестным размером, который требует точного размера
//  объем данных, хотим передовать владение, но с копированием
//  значенние, типаж которого нам важен, но не является конкретным типом 

use rand::RngExt;
#[derive(Debug,Clone)]
enum BitcoinBlock {
    Block(i32, Box<BitcoinBlock>),
    Genesis,
}

impl BitcoinBlock {
    fn print_block(&self) {
        match self {
            BitcoinBlock::Genesis => println!("Genesis block!"),
            BitcoinBlock::Block(id, prevblock) => {
                println!("Block id: {id}");
                match &**prevblock {
                    BitcoinBlock::Genesis => println!("Previous block is genesis"),
                    BitcoinBlock::Block(id, _) => println!("Previous block id: {id}"),
                }
            }
        }
    }
}

struct Sheep{}
struct Dog{}
struct Cat{}
struct Horse{}

#[derive(PartialEq, PartialOrd, Debug)]
struct Coin {
    name: String,
    price: f64
}

/*impl PartialEq for Coin {
    fn eq(&self, other: &Coin) -> bool {
        &self.price == &other.price
    }
}*/
trait Animal {
    fn noise (&self) -> String;
    fn name(&self) -> &'static str;
}

impl Animal for Cat {
    fn noise (&self) -> String {
        String::from("Dyn")
    }
    fn name(&self) -> &'static str {
        "cat"
    }
}

impl Animal for Dog {
    fn noise (&self) -> String {
        String::from("Woof")
    }
    fn name(&self) -> &'static str {
        "dog"
    }
}
impl Animal for Sheep {
    fn noise (&self) -> String {
        String::from("Beee")
    }
    fn name(&self) -> &'static str {
        "sheep"
    }
}
impl Animal for Horse {
    fn noise (&self) -> String {
        String::from("Neigh")
    }
    fn name(&self) -> &'static str {
        "horse"
    }
}
use crate::BitcoinBlock::{Block, Genesis};

fn main() {
    let x = Box::new(7);
    println!("x = {x}");

    let genblock = Genesis;
    let firstblock = Block(1, Box::new(genblock.clone()));
    let secondblock = Block(2, Box::new(firstblock.clone()));
    
    firstblock.print_block();
    secondblock.print_block();

    println!("__________________________________________");

    let mut rng = rand::rng();
    let rand_num: f64 = rng.random_range(0.0..=5.0);
    let animal = random_animal_impl(rand_num);
    println!("Your animal is {}. It says: {}",animal.name(),animal.noise());

    println!("__________________________________________");

    let sheep = Sheep{};
    let dog = Dog{};
    let cat = Cat{};
    let horse = Horse{};

    let mut list:Vec<Box<dyn Animal>> = Vec::new();
    list.push(Box::new(sheep));
    list.push(Box::new(dog));
    list.push(Box::new(cat));
    list.push(Box::new(horse));

    for animal in &list {
        println!("{}: {}", animal.name(), animal.noise());
    }

    println!("__________________________________________");

    let bitcoin = Coin{name: "Bitcoin".to_string(), price: 45750.32};
    let dogecoin = Coin{name: String::from("Dogecoin"), price: 0.26};

    let is_eq = bitcoin == dogecoin;
    println!("{is_eq}");
    println!("{} is more expensive than {}: {}", bitcoin.name,dogecoin.name, bitcoin > dogecoin);// false т.к. сравнивает каждое поле 
    println!("{:?}", dogecoin);


}

fn random_animal_impl (ran_num: f64) -> Box<dyn Animal> { // экземпляр обязан иметь реализачию Animal
    if ran_num < 0.33 {
        Box::new(Sheep {})
    } else if  ran_num < 1.2 {
        Box::new(Dog {})
    } else if ran_num < 2.0 {
        Box::new(Cat {})
    } else {
        Box::new(Horse {})
    }
}

/*
в derive:
Типажи сравненияЖEq, PartialEq, Ord, PartialOrd
Clone, для создания Т из &T 
Cope, чтобы создать тип семантикой копирования, а не перемещать владение 
Hash, чтобы вычислить хэш из &T
Default, для создания пустого экземпляра
Debug, для вывода
*/