use std::thread;
use std::time::Duration;// задержка 
fn main(){
    // Threads 
    let data = vec![1,3,5,7,9,11,112,113];
    let handle = thread::spawn(||{
        for i in 1..=5{
            println!("Thread: {i}");
            thread::sleep(Duration::from_secs(1));
        }
    });
    
    let handle2 = thread::spawn(move ||{
        for i in data{
            println!("Vector: {i}");
            thread::sleep(Duration::from_secs(1));
        }
    });
    
    let data_2 = vec![2,3,4,5,6,6,7,7,87];

    let handle3 = thread::spawn({
        let data_clone = data_2.clone();
        move || {
            for i in data_clone{
                println!("Vector_2: {i}");
                thread::sleep(Duration::from_secs(3));
            }
        }
    });

    for i in data_2{
        println!("Main thread: {i}");
        thread::sleep(Duration::from_secs(2));

    }
    
    handle.join().unwrap(); // жде конец программы пока 2 поток не завиршиться
    handle2.join().unwrap();
    handle3.join().unwrap();

}