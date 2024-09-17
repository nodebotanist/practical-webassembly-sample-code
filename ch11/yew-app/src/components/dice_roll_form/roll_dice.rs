use wasm_bindgen::prelude::*;
use getrandom::getrandom;
use regex::Regex;
use js_sys::{WebAssembly, Reflect, Function, Object, Array};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::console;
use gloo_console::log;

#[derive(Debug)]
#[wasm_bindgen]
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

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "parse_roll_string")]
    fn parse_roll_string(roll_string: &str) -> js_sys::Array;
}

const WASM: &[u8] = include_bytes!("../../../lib/ch04/release.wasm");

async fn parse_roll_string_async(roll_string:&str) -> Result<JsValue, JsValue> {
    let a = JsFuture::from(WebAssembly::instantiate_buffer(WASM, &Object::new())).await?;
    let b: WebAssembly::Instance = Reflect::get(&a, &"instance".into())?.dyn_into()?;

    let c = b.exports();

    let parse_roll_string = Reflect::get(c.as_ref(), &"parse_roll_string".into())?
        .dyn_into::<Function>()
        .expect("parse_roll_string export wasn't a function");

    let result = parse_roll_string.call1(&JsValue::undefined(), &JsValue::from("7d8+9")).unwrap();

    Ok(result)
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
pub async fn roll_dice(roll_string: &str) -> Result<RollResult, JsError> {
    // make sure the roll is valid
    if !validate_roll_string(roll_string) {
        // throw the error out to JS
        return Err(JsError::new(&format!("Invalid roll string: {:?}", roll_string).to_owned()));
    }

    // parse the roll
    let roll_numbers = parse_roll_string_async(&roll_string).await.unwrap();

    web_sys::console::log_1(&JsValue::from(format!("{:?}", roll_numbers)));

    // use the JsArray get function to get the values, then parse into f64, saved as i32 values
    // let number_of_dice = roll_numbers.get(0).as_f64().unwrap() as i32;
    // let die_max = roll_numbers.get(1).as_f64().unwrap() as i32;
    // let modifier = roll_numbers.get(2).as_f64().unwrap() as i32;



    let result = roll_dice_from_numbers(3, 10, 5);

    // return the result
    Ok(RollResult{
        dice_results: Vec::<i32>::from([8, 7, 6]),
        total: 25
    })
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

#[wasm_bindgen]
pub async fn roll_dice_log(roll_string: &str) {
    // let result = roll_dice(roll_string);
    // console log
    use web_sys::console;

    match Ok::<&str, JsError>("") {
        Ok(roll_result) => console::log_1(&format!("Roll: {:?}, Result: {:?}", roll_string, roll_result).into()),
        Err(_) => panic!("Error getting the roll result")
    }
}