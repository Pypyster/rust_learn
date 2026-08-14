mod typestate;

pub use crate::typestate::*;

fn main() -> Result<(), Error> {
    let req_builder = RequestBuilder::new()
    .url("https://some-url.com")
    .method("Get");

    let req = req_builder
    .header("Token", "uuid.exp.sign")
    .build()?;

    println!("{req:#?}");

   /*  let req_builder_2 = RequestBuilder::new()
    .url("https://some-url.com");
    //.method("Get");

    let req_2 = req_builder_2
    .header("Token", "uuid.exp.sign")
    .build()?;

    println!("{req_2:#?}");
    
    let req_builder_3 = RequestBuilder::new()
    .url("https://some-url.com");
    //.method("Get");

    let req_3 = req_builder_3.seal()
    .header("Token", "uuid.exp.sign")
    .build()?;

    println!("{req_2:#?}");*/*/

    Ok(())

}