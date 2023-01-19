use std::env;
use std::process; //allow us to exit process without panic

use rust_mini_grep::Config;

//main.rs is binary crate; lib.rs is library crate
fn main() {

    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("Problem parsing argument: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = rust_mini_grep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}




