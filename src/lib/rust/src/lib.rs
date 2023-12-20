mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(thing: &str) {
    alert(&format!("Hello, {thing}!").to_owned());
}
