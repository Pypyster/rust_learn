use std :: fs;
use std :: io;

fn main (){
    let file_name = "output.txt";
    match write_to_file(file_name,"Hello Merphy ❤️ 👋 ") {
        Ok(()) => println!("Data is written!"),
        Err(e) => println!("Error: {e}"),
    }

    match read_file(file_name) {
        Ok (content) => println!("Read: {content}"),
        Err(e) => println!("Error: {e}"),
        
    }
}

fn write_to_file(file_path: &str, content: &str) -> Result<(), io :: Error>{
    fs :: write(file_path, content)
} 

fn read_file(file_path: &str) -> Result<String, io :: Error>{
    fs :: read_to_string(file_path)
}