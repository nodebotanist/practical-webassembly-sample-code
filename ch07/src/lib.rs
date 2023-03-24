mod utils;

extern crate serde;

use wasm_bindgen::prelude::*;
use rand::Rng;
use serde::{Serialize, Deserialize};


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Serialize, Deserialize, Debug)]
#[wasm_bindgen]
struct RollResult {
    total: i32,
    dice_results: Vec<i32>
}


#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn roll_die(die_max: i32) -> i32 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(1..(die_max +1))
}

#[wasm_bindgen]
pub fn roll_dice(number_of_dice: i32, die_max: i32, modifier: i32) -> String {
    let mut dice_result: Vec<i32> = Vec::new();
    let mut total: i32 = 0;
    for _n in 0..number_of_dice {
        let roll = roll_die(die_max);
        dice_result.push(roll);
        total += roll;
    }
    return serde_json::to_string(&RollResult {
        dice_results: dice_result,
        total: total + modifier
    }).unwrap();
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
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    val.set_text_content(Some(&result));

    body.append_child(&val)?;

    Ok(())
}
