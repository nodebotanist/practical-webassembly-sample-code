extern crate roll_checker;
use std::env;
use std::fs::OpenOptions;
use std::io::prelude::*;

#[no_mangle]
pub fn main() {
    roll_dice_log()
}

#[no_mangle]
pub extern "C" fn roll_dice_log() {
    let args: Vec<String> = env::args().collect();

    if args.len() >= 4 {
        let number_of_dice = args[1].clone().parse().unwrap();
        let die_max = args[2].clone().parse().unwrap();
        let modifier = args[3].clone().parse().unwrap();

        let result = roll_checker::roll_dice_from_numbers(number_of_dice, die_max, modifier);
        println!("Result of {:?}d{:?}+{:?}: {:?}", number_of_dice, die_max, modifier, result);

        let mut roll_log_file = OpenOptions::new()
            .write(true)
            .append(true)
            .open("./roll-log.txt")
            .unwrap();

        if let Err(e) = writeln!(roll_log_file, "Result of {:?}d{:?}+{:?}: {:?}", number_of_dice, die_max, modifier, result) {
            panic!("Error writing to file: {:?}", e);
        }
    } else {
        panic!("Need 3 integer arguments ## ## ##");
    }
}
