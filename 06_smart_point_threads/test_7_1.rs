use std::{cell::{Ref, RefCell}, rc::{Rc,Weak}};
use std::sync::{Arc,RwLock,Mutex,Condvar};
use std::thread;

struct DataBase{
    records: Vec<String>,
}

#[derive(Debug)]
struct Node {
    id: u16,
    next: RefCell<Option<Weak<Node>>>,
}

impl Drop for Node{
    fn drop(&mut self) {
        println!("Dropping {}", self.id);
    }
}

fn main() {
    let a  = Rc::new(Node{
        id: 1,
        next: RefCell::new(None),
    });
    let b = Rc:: new(Node{
        id: 2,
        next: RefCell::new(None),
    });

    *a.next.borrow_mut() = Some(Rc::downgrade(&b));
    *b.next.borrow_mut() = Some(Rc::downgrade(&a));

    println!("Strong a: {}", Rc::strong_count(&a));

    let db = Arc::new(RwLock::new(DataBase{
        records: vec![]
    }));
    let mut handles = vec![];
    for i in 0..5{
        let db_clone = Arc::clone(&db);
        handles.push(thread::spawn( move ||{
            let mut db_wr = db_clone.write().unwrap();
            db_wr.records.push(format!("Writes thread {}", i));
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
    let db_read = db.read().unwrap();
    println!("{:?}", db_read.records);

    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut ready = lock.lock().unwrap();
        *ready = true;
        cvar.notify_one();
    });

    let (lock, cvar) = &*pair;
    let mut ready = lock.lock().unwrap();
    while !*ready {
        ready = cvar.wait(ready).unwrap();
    }
    println!("Finished!");
}