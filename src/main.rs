use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3{
        panic!("Error: Expected three arguments, but received {}.", args.len()-1);
    }
    println!("Args: {:?}", args);
}
