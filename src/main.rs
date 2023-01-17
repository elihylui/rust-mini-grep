use std::env;
use std::fs;
use std::process; //allow us to exit process without panic
use std::error::Error;

//main.rs is binary crate; lib.rs is library crate
fn main() {

    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("Problem parsing argument: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text: \n{}", contents);

    Ok(())
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    //constructor function - named as new() as a convention
    fn new(args: &[String]) -> Result<Config, &str> {
        //error handling:
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        //takes 2 arguments: query & filename
        let query = args[1].clone(); //&args[0] is path to binaries
        let filename = args[2].clone();
    
        Ok(Config { query, filename })
    }
    
}

