use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let foldername = &args[1];

    let filename = format!("./src/{}/input.txt", foldername);
    
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

}
