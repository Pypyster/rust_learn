use std::sync::mpsc as std_mpsc;
use tokio::sync::mpsc;
use tokio::sync::{Mutex, Barrier};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
enum OfficeTeam{
    Coffee(String),
    Donut(u32),
    Warning(String),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (sender, mut receiver) = mpsc::channel(10);

    tokio::spawn(async move{
        println!("Start:");
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        let coffee = "Bumbl".to_string();

        if let Err(_) = sender.send(coffee).await{
            println!("Receiver left(");
        } else {
            println!("Send for post");
        }
    });

    while let Some(coffee) = receiver.recv().await {
        println!("Obtain {}", coffee);
        break;
    }

    let (tx, mut rx) = mpsc::channel::<OfficeTeam>(5);
    let tx1 = tx.clone();

    tokio::spawn(async move {
        tx1.send(OfficeTeam::Coffee("Double espresso".to_string()))
        .await.unwrap();
    });

    let tx2 = tx.clone();
    tokio::spawn(async move {
        tx2.send(OfficeTeam::Donut(3))
        .await.unwrap();
    });
    drop(tx);

    while let Some(item) = rx.recv().await  {
        match item {
            OfficeTeam::Coffee(name) => println!("Your coffee: {name}"),
            OfficeTeam::Donut(num) => println!("Your {num} donuts"),
            OfficeTeam::Warning(err) => println!("Sorry, {err}"),
        }
    }

    let (sender, receiver): (std_mpsc::Sender<String>, std_mpsc::Receiver<String>)
    = std_mpsc::channel();
        let scloned = sender.clone();
        thread::spawn(move||{
            let vals = vec![
                String::from("First msg from 1 channel"),
                String::from("Second msg from 1 channel"),
                String::from("Third msg from 1 channel"),
                String::from("Something new from 1 channel"),
                String::from("Let's go from 1 channel")
            ];

            for val in vals{
                sender.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });
        thread::spawn(move||{
            let vals = vec![
                String::from("First msg from 2 channel"),
                String::from("Second msg from 2 channel"),
                String::from("Third msg from 2 channel"),
                String::from("Something new from 2 channel"),
                String::from("Let's go from 2 channel")
            ];

            for val in vals{
                scloned.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

    for mes in receiver {
        println!("Message: {mes}");
    }

    Ok(())

}