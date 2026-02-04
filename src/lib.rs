use std::{error::Error, fs};

#[derive(Debug, PartialEq)]
pub enum Arguments {
    CaseInsensitive,
    None
} 

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub arguments: Arguments
}


impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 { return Err("not enough arguments") }

        let query = args[1].clone();
        let file_path = args[2].clone();
        
        let arguments = if args.len() > 3 {
            Config::check_arguments(&args[3])
        } else {
            Arguments::None
        };
    

        Ok(Config {query, file_path, arguments})
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>>{
        let contents = fs::read_to_string(&self.file_path)?;

        let results = if self.arguments == Arguments::CaseInsensitive {
            search_case_insesitive(&self.query, &contents)
        } else {
            search(&self.query, &contents)
        };
        
        for line in results {
            println!("{line}");
        }
    
        Ok(())
    }

    fn check_arguments(args: &String) -> Arguments {
        let mut arguments = Arguments::None;
        
        if args.contains("i") {
            arguments = Arguments::CaseInsensitive;
        }
        

        arguments
    }
    
}

pub fn search<'a>(
    query: &str,
    contents: &'a str
) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insesitive<'a>(
    query: &str,
    contents: &'a str
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results: Vec<&str> = Vec::new();
    
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.
Trust me.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

        #[test]
        fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insesitive(query, contents)
        );
    }

    #[test]
    fn arguments_case_insensitive() {
        let args = "i".to_string();
        let result = Arguments::CaseInsensitive;

        assert_eq!(Config::check_arguments(&args), result)
    }

}

