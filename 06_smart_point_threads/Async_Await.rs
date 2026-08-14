use tokio::time::sleep;
use std::time::Duration;// задержка 

async fn simulate_download(file_name: &str, sec: u32) -> String {
    println!("Starting downlods {file_name}");
    sleep(Duration::from_secs(sec.into())).await; // ждем пока это выполнится
    println!("Downloading is finished)");
    file_name.to_string()
}
#[tokio::main]
async fn main(){
    // Async & Await 
    let (task1,task2) = tokio::join!(
        simulate_download("File 1", 3),
        simulate_download("Aloha chushpany",2)
    ); 
  
    println!("{task1} and {task2} are dowloaded!");
}   