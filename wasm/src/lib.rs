use wasm_bindgen::prelude::*;

// import the `alert` function from the Web (javascript)
#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}", name));
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) {
    log(&format!("{}", a + b))
}
