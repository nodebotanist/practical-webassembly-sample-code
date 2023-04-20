mod utils;

use std::slice;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use rand::Rng;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Debug)]
#[wasm_bindgen]
pub struct RollResult {
    total: i32,
    dice_results: Vec<i32>
}

#[wasm_bindgen]
impl RollResult {
    pub fn new(dice_results:Vec<i32>, total: i32) -> RollResult {
        RollResult {
            dice_results: dice_results,
            total: total
        }
    }

    pub fn get_total(&self) -> i32 {
        self.total
    }
}

#[wasm_bindgen]
pub fn roll_die(die_max: i32) -> i32 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(1..(die_max +1))
}

#[wasm_bindgen]
pub fn roll_dice(number_of_dice: i32, die_max: i32, modifier: i32) -> RollResult {
    let mut dice_result: Vec<i32> = Vec::new();
    let mut total: i32 = 0;
    for _n in 0..number_of_dice {
        let roll = roll_die(die_max);
        dice_result.push(roll);
        total += roll;
    }
    return RollResult {
        dice_results: dice_result,
        total: total + modifier
    };
}

pub fn parse_roll(roll_string: &str) -> [i32;3] {
    let roll_split = roll_string.split(['d', '+']);
    let mut result: [i32;3] = [0, 0, 0];
    let mut i: usize = 0;
    for dice in roll_split {
        result[i] = dice.parse::<i32>().unwrap();
        i+=1;
    }
    return result;
}

#[wasm_bindgen]
pub fn print_result_to_dom(dice_roll: String) -> Result<(), JsValue> {
    let dice_to_roll = parse_roll(&dice_roll);
    let result = roll_dice(dice_to_roll[0], dice_to_roll[1], dice_to_roll[2]);
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    val.set_text_content(Some(&format!("Result: {:?} Dice: {:?}", result.total, result.dice_results)));

    body.append_child(&val)?;

    Ok(())
}

#[wasm_bindgen]
pub fn send_rust_object_back(dice_roll: String) -> RollResult {
    let dice_to_roll = parse_roll(&dice_roll);
    return roll_dice(dice_to_roll[0], dice_to_roll[1], dice_to_roll[2]);
}
