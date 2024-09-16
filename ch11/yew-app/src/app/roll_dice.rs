use wasm_bindgen::prelude::*;
use getrandom::getrandom;
use regex::Regex;

#[derive(Debug)]
pub struct RollResult {
    pub total: i32,
    dice_results: Vec<i32>
}

impl RollResult {
    pub fn new(total:i32, dice_results:Vec<i32>) -> RollResult {
        RollResult {
            total: total,
            dice_results: dice_results
        }
    }
   
    pub fn get(&self) -> RollResult {
        return RollResult {
            total: self.total,
            dice_results: self.dice_results.clone()
        }
    }
}

pub fn roll_die(die_max: i32) -> i32 {
    // generate a random number between 1 and die_max inclusively
    let mut rand:[u8;1] = [0];
    let random_result = getrandom(&mut rand);
    match random_result {
        Ok(()) => {
            // rand[0] is now a random u8. To get the range, we divide by the max value and multiply by die_max
            let ratio = rand[0] as f32 / u8::MAX as f32;
            let mut result = (ratio * die_max as f32).ceil() as i32;
            if result == 0 { result = 1; }
            result
        }
        Err(_) => panic!("Error getting random bytes")
    }

}

pub fn roll_dice_from_numbers(number_of_dice:i32, die_max:i32, modifier:i32) -> RollResult {
    let mut result:i32 = 0;
    let mut rolls: Vec<i32> = Vec::new();
    for _ in 1..number_of_dice {
        let roll = roll_die(die_max);
        result += roll;
        rolls.push(roll);
    }
    result += modifier;
    RollResult::new(result, rolls)
}

pub fn main() {
    return
}
