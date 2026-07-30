use std::slice::Iter;

use reqwest::Error;
#[derive(Clone,Debug)]
struct Person {
    name: String,
    age: u16,
}


struct PersonIter<'a> {
    people: &'a [Person],
    index: usize,
}

impl<'a> Iterator for PersonIter<'a> {
    type Item = &'a Person;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.people.len() {
            let person = &self.people[self.index];
            self.index += 1;
            Some(person)
        } else {
            None
        }
    }
}

fn iter_struct(people: &[Person]) -> PersonIter<'_> {
    PersonIter { people, index: 0 }
}

fn filter_name(people: &[Person]) -> Vec<Person> {
    iter_struct(people)
        .filter(|p| p.name.len() > 3)
        .cloned()
        .collect()
}


fn main () {
    //into_iter итератор по значению 
    let v = vec![1,2];
    let mut iter = v.into_iter();
    let first = iter.next();
    let second = iter.next().expect("this element doesn't exist");
    let third = iter.next();//.expect("this element doesn't exist"); None
    println!("second: {:#?}",second);
    println!("third: {:#?}",third);
    
    let mut v = vec![1,2,3,4,5,6,7,8,9];
    let mut iter = v.into_iter();
    for i in iter {
        println!("i: {}",i);
    }
    //println!("First element: {}", v[0]); т.к. into_iter забрал владение

    //iter по немутабельным ссылкам 
    let v = vec![1,2,3,4,5,6,7];
    let ref_iter = v.iter();// передает ссылку на объект
    for i in ref_iter {
        println!("i: {}",i);
    }
    println!("First element is {}",v[0]); 

    //iter_mut указатель по мутабельным ссылкам 
    let mut  v:Vec<f32> = vec![1.0,2.0,3.0,4.0,5.0,6.0,7.0];
    let ref_iter = v.iter_mut();// передает ссылку на объект
    for i in ref_iter {
        *i *= 1.0/2.0;
        println!("i: {}",i);
    }
    println!("First element is {}",v[0]); 

    // map принимает замыкание и возвращает итератор 
    let mut v = vec![2,4,6,8,10,12,14,18];
    let new_vec: Vec<_> = v.iter().map(|x| *x as f32 /3.0).collect();
    println!("{:#?}", new_vec);
    println!("First el in new_vec is {}",new_vec[0]);

    // filter  свой 
    let vector = vec![1,2,3,4,5,6,7,8,9,10];
    let new_vector = filter_greater_than (vector, 6);
    assert_eq!(new_vector, vec![ 7,8, 9, 10]);// либо () либо panic


    let  v: Vec<i32> = (1..101).collect();
    println!("{:#?}", even_vec(&v));
    println!("First el is {}",even_vec(&v)[0]);

    let v: Vec<Vec<i32>> = vec![vec![1,2,3,4,5,6],vec![0,-1,-2,-3,-4,-5],vec![4,8,-12]];
    println!("Split into 1D: {:?}", simple(v));

    let v1 = vec![1,2,3,4,5,6,7,88,9,0];
    let mut v2 = v1.clone();
    v2.pop();
    if let Ok(res_1) = make_pair_vec(&v1){
        println!("{:?}", res_1);
    }
    match make_pair_vec(&v2){
        Ok(res) => println!("Sum pairs: {:#?}", res),
        Err(e) => println!("Error: {}", e),
    }

    let people = vec![
    Person { name: "Ann".to_string(), age: 20 },
    Person { name: "Bob".to_string(), age: 21 },
    ];
    let mut vec_people = vec![
        Person{name: "Merphy".to_string(), age: 8},
        Person{name: String::from("Milena"), age: 31},
        Person{name: "Kri".to_string(), age: 20},
        Person{name: "Ray".to_string(),age: 54 }
    ];

    for person in iter_struct(&people) {
        println!("{} {}", person.name, person.age);
    }

    println!("People with name length more 3: {:?}", filter_name(&vec_people));

    let n1 = &people[0];
    println!("{:p}", n1);
}

fn filter_greater_than (vec: Vec<i32>, value: i32) -> Vec<i32>{
    vec.into_iter().filter(|x| *x > value).collect() 
}

fn even_vec (vec: &[i32]) -> Vec<i32> {
    vec.into_iter().filter(|x| **x%2 == 0).copied().collect()
}

fn simple (vec: Vec<Vec<i32>>) -> Vec<i32> {
    vec.into_iter().flatten().collect()
}

fn make_pair_vec (vec: &[i32]) -> Result<Vec<i32>, String> {
    if vec.len() % 2 != 0 {
        Err("Vector has an odd number of elements".to_string())
    } else{
        Ok(vec.chunks(2)
        .map(|chunk| chunk.iter().copied().sum())
        .collect())
    }
}

