mod utils;

// this brings in wasm-bindgen, which will allow us to interact with JS APIs
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This decoorator exposes the function and JS to each other, allowing us to make console.log calls
#[wasm_bindgen]
pub fn attackroll(name: &str, dice_roll: i32, health: i32, hurt: bool)-> String {
    // this imports the console.log function from the web_sys crate
    use web_sys::console;

    let mut total: i32 = if hurt { health - dice_roll } else { health + dice_roll };
    if hurt && (health < dice_roll) { total = 0; }

    // here's our console.log call to the web_sys crate
    console::log_1(&format!("{} took {} {} and has {} health", name, dice_roll, if hurt {"damage"} else {"healing"}, total).into());
    format!("{} took {} {} and has {} health", name, dice_roll, if hurt {"damage"} else {"healing"}, total)
}