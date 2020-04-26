
include!("main.rs");

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/main.js")]
extern "C" {
    pub fn get_input_math_function() -> String;
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue>
{
    console_log::init_with_level(log::Level::Debug).unwrap();

    use log::info;
    info!("Logging works!");

    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    start_main(get_input_math_function);
    Ok(())
}
