use std::error::Error;
use std::{fs, result};


pub struct Config {
    query: String,
    file_path: String,
}

impl Config {

    pub fn build (args: &[String]) -> Result<Config, &'static str> { // I'm really loving this tutorial. It's teaching me how to write code in a rust-like way!
        
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query:String = args[1].clone(); // String to find in the file
        let file_path:String = args[2].clone(); // path to the file to be searched.
    
        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{


    let contents = fs::read_to_string(config.file_path)?;

    println!("With text: \n{contents}");

    Ok(())

}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query,contents)); // Comparing these two outcomes with asser_eq!
    }
}
