use tokio::time;
use log::Level;
use tokio::io::{AsyncReadExt,AsyncWriteExt};
use tokio::{net,task};
use std::str;

async fn sleeper() {
    log::info!("Sleeping");
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    log::info!("Awake!");
}

async fn reader() {
    log::info!("Reading some data");
    let mut f = tokio::fs::File::open("Lx_1.0_H2.csv").await.unwrap();
    let mut content = vec![];
    f.read_to_end(&mut content).await.unwrap();
    log::info!("Read {} bytes", content.len());
}


async fn run() {
    tokio::join!(
        sleeper(),
        reader(),
    );
}

fn fib(n: u64) -> u64{
    if n <= 1 {
        n
    } else {
        fib(n-1) + fib(n-2)
    }
}

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(Level::Info).unwrap();

    //let rt = tokio::runtime::Runtime::new().unwrap();// создает исполняющую среду
    /*let future = run();
    rt.block_on(future);*/
    run().await;

    let host = "localhost:8080";

    let srv = net::TcpListener::bind(host).await.unwrap();
    let mut count = 2;
    loop{
        count -=1;
        let (mut sock, _) = srv.accept().await.unwrap();

        tokio::spawn(async move {
            let mut buf = [0;124];
            let n = sock.read(&mut buf).await.unwrap();
            sock.write_all(&buf[0..n]).await.unwrap();

            let data =  str::from_utf8(&buf[0..n]).unwrap();
            println!("Echoed: {:?}", data);
            sock.shutdown().await.unwrap();
        });

        if count == 0 {
            break;
        }
    }
    
    let a = task::spawn_blocking(||{
        println!("Fib(40) computation");
        let res = fib(40);
        println!("Finished: {res}");
    });
    let b = task::spawn_blocking(||{
        println!("Fib(17) computation");
        let res = fib(17);
        println!("Finished: {res}");
    });

    tokio::join!(a,b).0.unwrap();
    
}