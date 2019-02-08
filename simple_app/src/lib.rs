#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate wasm_bindgen;

use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::WebGlRenderingContext as GL;

/// Used to run the application from the web
#[wasm_bindgen]
pub struct WebClient {
//    gl: Rc<WebGlRenderingContext>,
}
#[wasm_bindgen]
impl WebClient {

    /// Create a new web client
    #[wasm_bindgen(constructor)]
    pub fn new() -> WebClient {

        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
//
//        let canvas: HtmlCanvasElement = document.create_element("canvas").unwrap().dyn_into()?;
//
//        canvas.set_width(CANVAS_WIDTH as u32);
//        canvas.set_height(CANVAS_HEIGHT as u32);
//
//        let app_div: HtmlElement = match document.get_element_by_id(APP_DIV_ID) {
//            Some(container) => container.dyn_into()?,
//            None => {
//                let app_div = document.create_element("div")?;
//                app_div.set_id(APP_DIV_ID);
//                app_div.dyn_into()?
//            }
//        };
//
//        app_div.style().set_property("display", "flex")?;
//        app_div.append_child(&canvas)?;
//
//        Ok(canvas)

//        let app = Rc::new(App::new());
//
//        let gl = Rc::new(create_webgl_context(Rc::clone(&app)).unwrap());
//        append_controls(Rc::clone(&app)).expect("Append controls");
//
//        let renderer = WebRenderer::new(&gl);
//
        WebClient {}
    }

    /// Start our WebGL Water application. `index.html` will call this function in order
    /// to begin rendering.
    pub fn start(&self) -> Result<(), JsValue> {
//        let gl = &self.gl;
//
        Ok(())
    }

    /// Update our simulation
    pub fn update(&self, dt: f32) {
        console::log_1(&JsValue::from("update"));
    }

    /// Render the scene. `index.html` will call this once every requestAnimationFrame
    pub fn render(&mut self) {
        console::log_1(&JsValue::from("render"));
    }
}
