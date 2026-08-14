use::std::{sync::{Arc,RwLock},thread,time::Duration};

fn write(data: Arc<RwLock<i32>>) {
 
    let mut data_lock = data.write().unwrap();
    for pass in 1..4 {
        *data_lock += 1;        // изменяем значение
        println!("Writer (Pass{}) - value: {}", pass, *data_lock);
        thread::sleep(Duration::from_millis(400)); 
    }
}
fn read(id: u32, data: Arc<RwLock<i32>>) {
    let value = data.read().unwrap();
    for pass in 1..4 {
        println!("Reader {} (Pass{}) - value: {}", id, pass, value);
        thread::sleep(Duration::from_millis(200)); 
    }
}
fn main() {
    let data = Arc::new(RwLock::new(5));// несколько потоков могут видеть данные, но только один меняет
    let mut handles = vec![];

    for i in 0..=4{
        let data_clone = Arc::clone(&data);
        let handl = thread::spawn(move||{
            let num = data_clone.read().unwrap();
            println!("Thread {i} read {}",*num);
            thread::sleep(Duration::from_secs(2));
        });
        handles.push(handl);
    }
    let writer = thread::spawn({
        let data_clone = Arc::clone(&data);
        move || {
            let mut num = data_clone.write().unwrap();
            *num *= 3;
            println!("Changed num");
        }
    });
    handles.push(writer);
    for handl in handles {
        handl.join().unwrap();
    }
    println!("Result number: {}", *data.read().unwrap());

    let value = Arc::new(RwLock::new(0));   
    let mut threads = vec![];
 
    let value_copy = Arc::clone(&value);
    // создаем поток писателя
    let writer = thread::spawn(move || write(value_copy));
    threads.push(writer);
 
    for id in 1..4 {
 
        let value_copy = Arc::clone(&value);
        // создаем поток читателя
        let reader = thread::spawn(move || read(id, value_copy));
 
        threads.push(reader);
    }
 
    for t in threads {
 
        t.join().unwrap();
    }
}