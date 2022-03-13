mod utils;

use wasm_bindgen::prelude::*;
use domain::*;

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
pub struct Entity {}

impl Entity{
    pub fn add(a: i32, b:i32) -> i32{
        domain::Entity::add(a, b)
    }
}


#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm!");
}
