use wasm_bindgen::prelude::*;
use getrandom;
use regex::Regex;

#[no_mangle]
#[wasm_bindgen]
pub extern "C" fn roll_dice(roll: String) -> String{
        let roll_numbers = validate_roll(&roll);
        let roll_results = calculate_roll(roll_numbers[0], roll_numbers[1], roll_numbers[2]);
        return format!("{:?}", roll_results)
}

pub fn calculate_roll(number_of_dice:i32, die_max:i32, modifier: i32) -> Vec<i32> {
    let mut result = Vec::<i32>::new();
    let mut roll_total = 0;

    fn roll_die() -> Result<i32, getrandom::Error> {
        let mut buf = [0u8; 1];
        getrandom::getrandom(&mut buf)?;
        Ok(i32::from(buf[0]))
    }

    for _i in 0..number_of_dice {
        let die_result: u8 = roll_die().unwrap() as u8;
        let intermediate = die_result as f32 / std::u8::MAX as f32;
        let mut die_result = die_max as f32 * intermediate;
        die_result = intermediate.ceil(); 
        result.push(die_result as i32);
        roll_total += die_result as i32;
    }
    roll_total += modifier;
    result.push(roll_total);
    result
}

pub fn validate_roll(roll: &str) -> [i32;3]{
    let roll_regex = Regex::new(r"^([0-9]+)d([0-9]+)\+?([0-9]+)?$").unwrap();
    let regex_captures = match roll_regex.captures(roll){
        None => return [0,0,0],
        Some(s) => s
    };
    let number_of_dice: i32 = regex_captures.get(1).map_or(0, |m| m.as_str().parse::<i32>().unwrap());
    let die_max = regex_captures.get(2).map_or(0, |m| m.as_str().parse::<i32>().unwrap());
    let modifier = regex_captures.get(3).map_or(0, |m| m.as_str().parse::<i32>().unwrap());
    [ number_of_dice, die_max, modifier]
}
