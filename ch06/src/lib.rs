mod utils;

use wasm_bindgen::prelude::*;
use getrandom::getrandom;
use regex::Regex;

#[wasm_bindgen(module = "/lib/ch04/release.js")]
extern "C" {
    #[wasm_bindgen(js_name = "parse_roll_string")]
    fn parse_roll_string(roll_string: &str) -> js_sys::Array;
}

#[wasm_bindgen]
pub fn validate_roll_string(roll_string: &str) -> bool {
    // upgrade this to use a rust regex
    let reg_exp = Regex::new(r"^[0-9]+[d][0-9]+((\+|-)[0-9]+)?$").unwrap();
    match reg_exp.captures(roll_string) {
        None => false,
        _ => true
    }
}

fn roll_die(die_max: i32) -> i32 {
    // generate a random number between 1 and die_max inclusively
    let mut rand:[u8;1] = [0];
    let random_result = getrandom(&mut rand);
    match random_result {
        Ok(()) => {
            // rand[0] is now a random u8. To get the range, we divide by the max value and multiply by die_max
            let ratio = rand[0] as f32 / u8::MAX as f32;
            let result = (ratio * die_max as f32).ceil() as i32;
            result
        }
        Err(_) => panic!("Error getting random bytes")
    }

}

#[wasm_bindgen]
pub fn roll_dice(roll_string: &str) -> Result<i32, JsError> {
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

    // start a running total of the dice roll
    let mut total = 0;

    // roll the dice
    for _ in 0..number_of_dice {
        total += roll_die(die_max);
    }

    // add the modifier
    total += modifier;

    // return the total
    Ok(total)
}

#[wasm_bindgen]
pub fn roll_dice_log(roll_string: &str) {
    let result = roll_dice(roll_string);

    match result {
        Ok(roll_result) => console::log_1(&format!("Roll: {:?}, Result: {:?}", roll_string, roll_result).into()),
        Err(_) => panic!("Error getting the roll result")
    }
    // console log result
    use web_sys::console;
    
}