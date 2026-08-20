use std::time::Duration;
use std::sync::Arc;
use tokio::sync::{oneshot, Mutex, Barrier};
use tokio::time::sleep;


#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cup = Arc::new(Mutex::new(6));
    let cup1 = Arc::clone(&cup);
    let task1 = tokio::spawn(async move {
        println!("[task1] trying to lock...");
        let mut sips = cup1.lock().await;
        println!("[task1] got lock!");
        sleep(Duration::from_secs(2)).await; 
        *sips -= 1;
        println!("[task1] Sips left: {}", *sips);
    });

    let cup2 = Arc::clone(&cup);
    let task2 = tokio::spawn(async move {
        sleep(Duration::from_millis(100)).await; 
        println!("[task2] trying to lock...");
        let mut sips = cup2.lock().await; 
        println!("[task2] got lock!");
        *sips -= 1;
        println!("[task2] Sips left now: {}", *sips);
    });
    
    println!("Arc: {}", Arc::strong_count(&cup));
    let _ = tokio::join!(task1,task2);

    println!("Arc: {}", Arc::strong_count(&cup));

    let barier = Arc::new(Barrier::new(4));

    let b1 = Arc::clone(&barier);
    tokio::spawn(async move {
        println!("First worker to place");
        b1.wait().await;
        println!("First start work");
    });
    let b2 = Arc::clone(&barier);
    tokio::spawn(async move {
        println!("Second worker to place");
        b2.wait().await;
        println!("Second start work");
    });
    let b3 = Arc::clone(&barier);
    tokio::spawn(async move {
        println!("Third worker to place");
        b3.wait().await;
        println!("Third start work");
    });

    barier.wait().await; //т.к. 4 участник блочит главную ветку
    println!("All work");

    let (tx, rx) = oneshot::channel();
    tokio::spawn(async move{
        tx.send("Problem was solved").unwrap();
    });

    match rx.await{
        Ok(msg) => println!("You send: {msg}"),
        Err(_) => println!("Not found sender"),
    }
    Ok(())
}