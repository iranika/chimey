//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
extern crate wasm;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    //assert_eq!(domain::add(1, 2), 3);
    assert_eq!(wasm::Entity::add(1, 2), 3);
    //assert_eq!(wasm::add())
}

