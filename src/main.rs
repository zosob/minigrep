use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3{
        panic!("Error: Expected three arguments, but received {}.", args.len()-1);
    }

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for: {}", query);
    println!("In file: {}", filename);


    let contents = match read_file(filename){
        Ok(c) => c,
        Err{e) =>{
            eprintln!("Error reading file: {}", e);
            process::exit(1);
        }}
    };

    println!("File contents: \n{}", contents);
}

fn read_file(filename: &str) -> Result<String, std::io::Error>{
    fs::read_to_string(filename)
}