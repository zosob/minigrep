use std::env;
use std::fs;
use std::process;

struct Config{
    query: String,
    filename: String,
    case_insensitive: bool,
}

impl Config{
    fn new(args: &[String]) -> Result<Config, &str>{
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        let case_insensitive = std::env::var("IGNORE_CASE").is_ok();

        Ok(Config{
            query,
            filename,
            case_insensitive,
        })
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments:{}", err);
        process::exit(1);
    });


    let contents = fs::read_to_string(&config.filename).unwrap_or_else(|err| {
        eprintln!("Error reading file: {}", err);
        process::exit(1);
    });

    //println!("File contents: \n{}", contents);

    let results = if config.case_insensitive{
        search_case_insensitive(&config.query, &contents)
    } else{
            search(&config.query, &contents)
    };
    for line in results{
        println!("{}", line);
    } 
}

//fn read_file(filename: &str) -> Result<String, std::io::Error>{
//    fs::read_to_string(filename)
//}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[test]
fn case_insensitive(){
    let query = "rUsT";
    let contents = "\
    Rust:
safe, fast, productive.
Trust me.";

    assert_eq!(
        search_case_insensitive(query, contents),
        vec!["Rust:", "Trust me."]
    );

}