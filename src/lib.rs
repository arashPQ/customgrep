use std::error::Error;
use std::fs;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let content = fs::read_to_string(config.file_name)?;
    

    let results = if config.case_sensitive {
        search(&config.query, &content)
    } else {
        search_case_insensitive(&config.query, &content)
    };


    for line in results{
        println!("\x1b[35m{}\x1b[0m", line);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_name: String,
    pub case_sensitive: bool,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {

        if args.len() < 3 {
            return Err("Not enough arguments!!");
        }

        let query = args[1].clone();
        let file_name = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {query , file_name, case_sensitive})
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    
    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }

    return results;
}


pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    
    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }

    return results;
}




#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive(){
        let query = "arash";
        let contents = "this project developed by arash PQ.";

        assert_eq!(vec!["this project developed by arash PQ."],
         search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "dEV";
        let contents = "\
Developer:
This project developed by arash .";

        assert_eq!(
            vec!["Developer:", "This project developed by arash ."],
            search_case_insensitive(query, contents));
    }
}