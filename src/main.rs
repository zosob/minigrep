use std::env;
use std::process;

use minigrep::Config;
use clap::Parser;



fn main() {
    
    let config = Config::parse();
    
    if let Err(e) = minigrep::run(config){
        eprintln!("Application Error: {e}");
        process::exit(1);
    }
}

//fn read_file(filename: &str) -> Result<String, std::io::Error>{
//    fs::read_to_string(filename)
//}


