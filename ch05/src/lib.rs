mod utils;

use wasm_bindgen::prelude::*;
use js_sys::RegExp;

#[wasm_bindgen]
pub fn check_roll(roll_string: &str) -> bool {
    let reg_exp = js_sys::RegExp::new(r"/^[0-9]+[d][0-9]+(\+[0-9]+)?$/", "");
    return reg_exp.test(roll_string)
}

#[wasm-bindgen]
pub fn roll_dice()