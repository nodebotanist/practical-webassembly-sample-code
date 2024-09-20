extern crate roll_checker;
use std::env;
use std::fs::OpenOptions;
use std::io::prelude::*;

#[no_mangle]
pub fn main() {
    // this is the function you'll invoke from the CLI
    roll_dice_log()
}

#[no_mangle]
pub extern "C" fn roll_dice_log() {
    // get the CLI arguments sent in [0 is the filename]
    let args: Vec<String> = env::args().collect();

    // we want 2-3 arguments
    if args.len() >= 3 {
        let number_of_dice = args[1].clone().parse().unwrap();
        let die_max = args[2].clone().parse().unwrap();
        let modifier:i32 = if args.len() == 3 { 0 } else { args[3].as_str().parse().unwrap() };

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
        panic!("Need 2-3 integer arguments ## ## [##]");
    }
}
