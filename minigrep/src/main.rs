use std::{env, process};

use minigrep::Config;

fn main() {

    let args: Vec<String> = env::args().collect();
    

    let config = Config::build(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = Config::run(&config) {
        println!("Application error: {e}");
        process::exit(1);
    }
    

    println!("searching for {}", config.query);
    println!("path {}",config.file_path);
    // dbg!(args);
}



