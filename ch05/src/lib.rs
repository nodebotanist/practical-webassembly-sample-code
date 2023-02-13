mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn attackroll(name: &str, dice_roll: i32, health: i32, hurt: bool)-> String {
    let mut total: i32 = if hurt { health - dice_roll } else { health + dice_roll };
    if hurt && (health < dice_roll) { total = 0; }

    format!("{} took {} {} and has {} health", name, dice_roll, if hurt {"damage"} else {"healing"}, total)
}
