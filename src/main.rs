use std::env;
use std::error::Error;
use std::fs;
use std::process;
fn main() {
    let args:Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err |{
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_name);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}


fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let content = fs::read_to_string(config.file_name)?;
    println!("with text:\n{}", content);
    Ok(())
}



struct Config {
    query: String,
    file_name: String,
}
impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {

        if args.len() < 3 {
            return Err("Not enough arguments!!");
        }

        let query = args[1].clone();
        let file_name = args[2].clone();

        Ok(Config {query , file_name})
    }
}