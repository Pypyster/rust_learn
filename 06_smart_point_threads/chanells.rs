use std::sync::mpsc;
use std::sync::mpsc::{Sender,Receiver};
use std::thread;
use std::time::Duration;

fn main(){
    let (sender, receiver): (Sender<String>, Receiver<String>) 
        = mpsc::channel();
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

}