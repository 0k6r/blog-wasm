#![recursion_limit = "256"]
#![allow(clippy::eval_order_dependence)]

#[macro_use]
extern crate cfg_if;

extern crate wasm_bindgen;
extern crate web_sys;
extern crate yew;
extern crate yew_router;

use wasm_bindgen::prelude::*;

mod app;
mod component;

pub use app::App;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

// Called by our JS entry point to run the example
#[wasm_bindgen]
pub fn run() {
    // If the `console_error_panic_hook` feature is enabled this will set a panic
    // hook, otherwise it will do nothing.
    web_logger::init();

    yew::start_app::<App>();
}
