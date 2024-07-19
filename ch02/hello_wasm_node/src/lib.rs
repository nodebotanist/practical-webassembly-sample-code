// bring in the wasm-bindgen function decorator
use wasm_bindgen::prelude::*;

// this tells the compiler to make this function available to Javascript
#[wasm_bindgen]
pub fn hello_world() {
    // pull in the console API from the web-sys library within the wasm-bindgen library (more in chapter 7).
    use web_sys::console;
    // console.log("hello, world!");
    console::log_1(&"Hello, world!".into())
}