mod utils;

use wasm_bindgen::prelude::*;
use log::{info};

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
pub fn greet() {
    console_log::init()
        .expect("Unable to start logging");
    utils::set_panic_hook();

    info!("Oh hi, how's it going?");
    // alert("Hello, pack-test!");
}
