mod utils;

use wasm_bindgen::prelude::*;
use getrandom::getrandom;
use regex::Regex;

#[derive(Debug)]
#[wasm_bindgen]
pub struct RollResult {
    pub total: i32,
    dice_results: Vec<i32>
}

impl RollResult {
    pub fn get_dice_rolls(&self) -> String {
        format!("{:?}", self.dice_results)
    }
}

#[wasm_bindgen(module = "/lib/ch04/release.js")]
extern "C" {
    #[wasm_bindgen(js_name = "parse_roll_string")]
    fn parse_roll_string(roll_string: &str) -> js_sys::Array;
}

#[wasm_bindgen]
pub fn validate_roll_string(roll_string: &str) -> bool {
    let reg_exp = Regex::new(r"^[0-9]+[d][0-9]+((\+|-)[0-9]+)?$").unwrap();
    match reg_exp.captures(roll_string) {
        None => false,
        _ => true
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

#[wasm_bindgen]
pub fn roll_dice(roll_string: &str) -> Result<RollResult, JsError> {
    // make sure the roll is valid
    if !validate_roll_string(roll_string) {
        // throw the error out to JS
        return Err(JsError::new(&format!("Invalid roll string: {:?}", roll_string).to_owned()));
    }

    // parse the roll 
    let roll_numbers = parse_roll_string(&roll_string);
    // use the JsArray get function to get the values, then parse into f64, saved as i32 values
    let number_of_dice = roll_numbers.get(0).as_f64().unwrap() as i32;
    let die_max = roll_numbers.get(1).as_f64().unwrap() as i32;
    let modifier = roll_numbers.get(2).as_f64().unwrap() as i32;

    let result = roll_dice_from_numbers(number_of_dice, die_max, modifier);

    // return the result
    Ok(result)
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
    RollResult {
        total: result,
        dice_results: rolls
    }
}

#[wasm_bindgen]
pub fn roll_dice_log(roll_string: &str) {
    let result = roll_dice(roll_string);
    // console log
    use web_sys::console;

    match result {
        Ok(roll_result) => console::log_1(&format!("Roll: {:?}, Result: {:?}", roll_string, roll_result).into()),
        Err(_) => panic!("Error getting the roll result")
    }
}

pub fn main() {

}