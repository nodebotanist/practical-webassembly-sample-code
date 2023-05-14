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
        let roll_results = calculate_roll(number_of_dice, die_max, modifier);
        println!("{:?}", roll_results);


    } else {
        panic!("Error: need an argument dice roll ##d##+##");
    }
}

pub fn calculate_roll(number_of_dice:i32, die_max:i32, modifier: i32) -> Vec<i32> {
    let mut result = Vec::<i32>::new();
    let mut roll_total = 0;

    let mut rng = thread_rng();

    for _i in 0..number_of_dice {
        let die_result = rng.gen_range(1..(die_max + 1));
        result.push(die_result);
        roll_total += die_result;
    }
    roll_total += modifier;
    result.push(roll_total);
    result
}