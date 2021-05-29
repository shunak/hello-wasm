extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

extern {
    pub fn alert(s: &str);
}

pub fn greet(name: &str){
    alert(&format!("Hello, {}!",name));
}








