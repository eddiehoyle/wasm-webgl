#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate wasm_bindgen;

use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::WebGl2RenderingContext as GL;


fn init_canvas() -> Result<HtmlCanvasElement, JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let canvas: HtmlCanvasElement = document.create_element("canvas").unwrap().dyn_into()?;

    canvas.set_width(320);
    canvas.set_height(240);

    let app_div: HtmlElement = match document.get_element_by_id("simple_app") {
        Some(container) => container.dyn_into()?,
        None => {
            let app_div = document.create_element("div")?;
            app_div.set_id("simple_app");
            app_div.dyn_into()?
        }
    };

    app_div.style().set_property("display", "flex")?;
    app_div.append_child(&canvas)?;

    Ok(canvas)
}

fn create_webgl_context() -> Result<WebGl2RenderingContext, JsValue> {
    let canvas = init_canvas()?;

    let gl: WebGl2RenderingContext = canvas
        .get_context("webgl2")?
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()?;

    gl.clear_color(0.0, 0.0, 0.0, 1.0);
    gl.enable(GL::DEPTH_TEST);

    Ok(gl)
}

/// Used to run the application from the web
#[wasm_bindgen]
pub struct WebClient {
    gl: Rc<WebGl2RenderingContext>,
}
#[wasm_bindgen]
impl WebClient {

    /// Create a new web client
    #[wasm_bindgen(constructor)]
    pub fn new() -> WebClient {

        let gl = Rc::new(create_webgl_context().unwrap());

        WebClient { gl }
    }

    /// Start our WebGL Water application. `index.html` will call this function in order
    /// to begin rendering.
    pub fn start(&self) -> Result<(), JsValue> {
        console::log_1(&JsValue::from("Starting web client..."));
        let gl = &self.gl;
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
