use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::WebGl2RenderingContext as GL;
use std::borrow::Borrow;

fn init_canvas() -> Result<HtmlCanvasElement, JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

//    let canvas: HtmlCanvasElement = document.create_element("canvas").unwrap().dyn_into()?;
//
//    canvas.set_width(320);
//    canvas.set_height(240);
//    canvas.style().set_property("display", "inline");
//    canvas.style().set_property("float", "left");
//    canvas.style().set_property("padding-left", "10px")?;

//    let app_div: HtmlElement = match document.get_element_by_id("simple_app") {
//        Some(container) => container.dyn_into()?,
//        None => {
//            let app_div = document.create_element("div")?;
//            app_div.set_id("simple_app");
//            app_div.dyn_into()?
//        }
//    };
//
//    app_div.style().set_property("display", "flex")?;
//    app_div.append_child(&canvas)?;

    let canvas: HtmlCanvasElement = document.get_element_by_id("viewport").unwrap().dyn_into()?;
    canvas.set_width(320);
    canvas.set_height(240);
    canvas.style().set_property("display", "inline")?;
    canvas.style().set_property("float", "left")?;

    Ok(canvas)
}

pub fn create_webgl_context() -> Result<GL, JsValue> {
    let canvas = init_canvas()?;
    let gl: GL = canvas
        .get_context("webgl2")?
        .unwrap()
        .dyn_into::<GL>()?;
    gl.clear_color(0.0, 0.0, 0.0, 1.0);
    gl.enable(GL::DEPTH_TEST);
    Ok(gl)
}
