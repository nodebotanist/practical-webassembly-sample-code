mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add_two_numbers() {

    use web_sys::console;
    console::log_1(&"Hello, debugging-console-log!".into());
}
