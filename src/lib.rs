mod utils;

mod error;
mod types;

use wasm_bindgen::prelude::*;
use web_logger;

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
pub fn run_app() -> Result<(), JsValue> {
    alert("Hello, World!");
    web_logger::init();
    Ok(())
}
