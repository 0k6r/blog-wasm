#![recursion_limit = "1024"]
#![allow(clippy::eval_order_dependence)]

// extern crate wasm_bindgen;
// extern crate web_sys;
// extern crate yew;
// extern crate yew_router;

use wasm_bindgen::prelude::*;
use web_logger;

pub mod app;
pub mod components;
pub mod routes;

use app::App;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
// Called by our JS entry point to run the example

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    web_logger::init();
    yew::start_app::<App>();
    Ok(())
}
