use tree_sitter::Parser;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn foo() -> u64 {
    Parser::new().timeout_micros()
}
