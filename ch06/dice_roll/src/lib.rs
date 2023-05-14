use std::env;
use rand::prelude::*;
use regex::Regex;

#[no_mangle]
pub extern "C" fn roll_dice(){
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let roll = args[1].clone();

        let roll_regex = Regex::new(r"^([0-9]+)d([0-9]+)\+([0-9]+)$").unwrap();
        let regex_captures = match roll_regex.captures(&roll){
            None => panic!("No regex match"),
            Some(s) => s
        };
        let number_of_dice: i32 = regex_captures.get(1).map_or(0, |m| m.as_str().parse::<i32>().unwrap());
        let die_max = regex_captures.get(2).map_or(0, |m| m.as_str().parse::<i32>().unwrap());
        let modifier = regex_captures.get(3).map_or(0, |m| m.as_str().parse::<i32>().unwrap());
        println!("{}d{}+{}", number_of_dice, die_max, modifier);     



    } else {
        panic!("Error: need an argument dice roll ##d##+##");
    }
}

pub fn calculate_roll(number_of_dice:i32, die_max:i32, modifier: i32) -> Vec<i32> {
    return Vec::<i32>::new();
}