mod builder;
use crate::builder::{Server, TLSCest};

fn main() {
    let host = "localhost".to_owned();
    let port = 8080;

    let cert = TLSCest::new(
        "...".to_owned(),
        "<><>".to_owned(),
    );

    let basic = Server::new(host.clone(), port).build();

    let tls_server = Server::new(host.clone(), port).tls(cert.clone()).build();

    let server = Server::new(host, port).tls(cert).hot_reload(true).timeout(8741).build();

    println!("Bsic: {basic:#?}");
    println!("Tls: {tls_server:#?}");
    println!("Server: {server:#?}");
}