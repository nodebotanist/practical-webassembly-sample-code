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
struct Result {
    total: i32,
    dice_results: Vec<i32>
}


#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

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
    return serde_json::to_string(&Result {
        dice_results: dice_result,
        total: total + modifier
    }).unwrap()
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, ch07!");
}
