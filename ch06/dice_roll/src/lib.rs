use std::env;
use rand::prelude::*;
use regex::Regex;

#[no_mangle]
pub extern "C" fn roll_dice(){
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let roll = args[1].clone();
        let mut roll_numbers: [i32;3] = [0, 0, 0];

        let roll_split_dice = roll.split_once("d");
        if roll_split_dice.is_none() 
            || roll_split_dice.unwrap().0 == "" 
            || roll_split_dice.unwrap().1 == "" {
            panic!("Roll {} failed at split at d, make sure roll format is ##d##+##", roll);
        }

        let number_of_dice = match roll_split_dice.unwrap().0.parse::<i32>() {
            Ok(s) => s,
            Err(_e) => panic!("Errored out parsing the first number")
        };
        println!("{:?}", number_of_dice);
        roll_numbers[0] = number_of_dice;

        let roll_die_and_mod = roll_split_dice.unwrap().1.split_once("+");
        if roll_die_and_mod.is_none()
            || roll_die_and_mod.unwrap().0 == ""
            || roll_die_and_mod.unwrap().1 == "" {
            panic!("Roll {} failed at split on +, check that roll format is ##d##+##", roll);
        }

        let die_max = match roll_die_and_mod.unwrap().0.parse::<i32>(){
            Ok(s) => s,
            Err(_e) => panic!("Errored out parsing the second number")
        };
        roll_numbers[1] = die_max;

        let modifier = match roll_die_and_mod.unwrap().1.parse::<i32>(){
            Ok(s) => s,
            Err(_e) => panic!("Errored out parsing the third number")
        };
        roll_numbers[2] = modifier;        



    } else {
        panic!("Error: need an argument dice roll ##d##+##");
    }
}