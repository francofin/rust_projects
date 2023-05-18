use wasm_bindgen::prelude::*;


//exports file to webassembly.

#[wasm_bindgen]
pub fn greet(name: &str) {
    println!("Hi There {}", name);
}


//wasm-pack --target web