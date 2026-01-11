
use std::fs;
use std::error::Error;
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        println!("Searching for {}", self.query);
        println!("In file {}", self.filename);

        let contents = fs::read_to_string(&self.filename)?;

        println!("With text:\n{}", contents);

        Ok(())
        
    }
}
