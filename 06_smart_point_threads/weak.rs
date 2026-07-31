use crate:: BitcoinBlock::{Block, Genesis};
use std::{cell::{Ref, RefCell}, rc::{Rc,Weak}};

#[derive(Debug,Clone)]
enum BitcoinBlock {
    Block(i32, RefCell<Rc<BitcoinBlock>>),
    Genesis,
}

impl BitcoinBlock {
    fn get_prev_block(&self) -> Option<&RefCell<Rc<BitcoinBlock>>>{
        match self {
            Block(_, item) => Some(item),
            Genesis => None,
        }
    }
}

#[derive(Debug)]
struct SnakePart {
    id: u16,
    name: String,
    next: RefCell<Weak<SnakePart>>,
    prev: RefCell<Option<Rc<SnakePart>>>
}

fn main() {
    let block1 = Rc::new(Block(1, RefCell::new(Rc::new(Genesis))));
    println!("Block1 rc count: {}", Rc::strong_count(&block1));
    println!("Prev block: {:?}", block1.get_prev_block());

    let block2 = Rc::new(Block(1, RefCell::new(Rc::clone(&block1))));
    println!("Block1 rc count after new creation: {}", Rc::strong_count(&block1));
    println!("Block2 rc count: {}", Rc::strong_count(&block2));
    println!("Prev block: {:?}", block2.get_prev_block());

    if let Some(link) = block1.get_prev_block(){
        *(link.borrow_mut()) = Rc::clone(&block2);
    } // зациклились 

    println!("Block1 changed");
    println!("Block1 rc count: {}", Rc::strong_count(&block1));
    println!("Block2 rc count: {}", Rc::strong_count(&block2));

    let tail1 = Rc::new(SnakePart{
        id: 1,
        name: "tail 1".to_string(),
        next: RefCell::new(Weak::new()),
        prev: RefCell::new(Option::None),
    });

    println!("tail next: {:?}", tail1.next.borrow().upgrade());

    let head = Rc::new(SnakePart{
        id: 0,
        name: "head".to_string(),
        next: RefCell::new(Weak::new()),
        prev: RefCell::new(Option::Some(Rc::clone(&tail1))),
    });

    *tail1.next.borrow_mut() = Rc::downgrade(&head);
    println!("tail next: {:?}", tail1.next.borrow().upgrade());


}
