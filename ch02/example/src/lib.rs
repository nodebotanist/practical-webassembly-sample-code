// bring in the wasm_bindgen function decorator
use wasm_bindgen::prelude::*;

// tells the compiler to make this function accessible from JS
#[wasm_bindgen]
pub fn hello_world() {
    // pull in the console API from web-sys
    use web_sys::console;
    // console.log("Hello, world!")
    console::log_1(&"Hello, world!".into());
}
