use std::env;
use std::fs;

mod day01;

fn main() {
    let args: Vec<String> = env::args().collect();

    let foldername = &args[1];

    let filename = format!("./src/{}/input.txt", foldername);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut result =  format!("Invalid input: {}", foldername);

    if foldername == "day01" {
        result = format!("{:?}", day01::run(contents));
    }

    println!("{}", result);
}
