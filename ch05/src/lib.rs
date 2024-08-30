mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/lib/ch04/release.js")]
extern "C" {
    #[wasm_bindgen(js_name = "roll_die")]
    fn roll_die(dieMax: i32) -> i32;

    #[wasm_bindgen(js_name = "parse_roll_string")]
    fn parse_roll_string(roll_string: &str) -> js_sys::Array;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

fn validate_roll_string(roll_string: &str) -> bool {
    let reg_exp = js_sys::RegExp::new(r"^[0-9]+[d][0-9]+((\+|-)[0-9]+)?$", "");
    reg_exp.test(roll_string)
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
    console_log!("{}, {}, {}", roll_numbers.get(0).as_f64().unwrap(), roll_numbers.get(1).as_f64().unwrap(), roll_numbers.get(2).as_f64().unwrap());
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