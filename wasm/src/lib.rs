use wasm_bindgen::prelude::*;

// import the `alert` function from the Web (javascript)
#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}", name));
}
