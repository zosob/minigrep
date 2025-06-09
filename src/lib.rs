use std::fs;

pub struct Config{
    pub query: String,
    pub filename: String,
    pub case_insensitive: bool,
}

impl Config{
    pub fn new(args: &[String]) -> Result<Config, &str> {
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


pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>>{
    let contents = fs::read_to_string(config.filename)?;

    //println!("File contents: \n{}", contents);

    let results = if config.case_insensitive{
        search_case_insensitive(&config.query, &contents)
    } else{
            search(&config.query, &contents)
    };
    for line in results{
        println!("{}", line);
    } 
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
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