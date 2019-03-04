use wasm_bindgen::prelude::*;
use web_sys::*;
use web_sys::WebGl2RenderingContext as GL;


#[wasm_bindgen]
pub struct WebClient {
}

#[wasm_bindgen]
impl WebClient {

    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
        info!("New WebClient!");
        WebClient {}
    }

    pub fn start(&self) -> Result<(), JsValue> {
        info!("WebClient starting...");
        Ok(())
    }

    pub fn update(&self, delta: u32) {
    }

    pub fn render(&mut self) {
    }
}