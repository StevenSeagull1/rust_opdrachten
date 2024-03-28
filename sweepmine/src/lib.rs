mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

pub static DEFAULT_COLOR: &str = "black";
pub static HIT_COLOR: &str = "red";

#[wasm_bindgen]
pub struct Cell {
    #[wasm_bindgen(skip)]
    pub color: String,
}

#[wasm_bindgen]
impl Cell {
    #[wasm_bindgen(constructor)]
    pub fn new(color: &str) -> Self {
        Cell {
            color: String::from(color),
        }
    }

    pub fn hit(&mut self) {
        self.color = String::from(HIT_COLOR);
    }

    #[wasm_bindgen(getter)]
    pub fn color(&self) -> String {
        self.color.clone()
    }
}
