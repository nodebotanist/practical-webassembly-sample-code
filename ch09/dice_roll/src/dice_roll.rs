extern crate regex;
use std::env;
use regex::Regex;

fn main(){
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let roll = args[1].clone();

        let rollRegex = Regex::new(r"^[0-9]+[d][0-9]+(\+[0-9]+)?$").unwrap();
        if rollRegex.is_match(roll) {
            println!("Roll {} is valid", roll);
        } else {
            eprintln!("Roll {} is invalid, must be ##d##+##", roll);
        }
        
    } else {
        eprintln!("Error: need an argument dice roll ##d##+##");
    }
}