use std::cell::RefCell;
use std::rc::Rc;
use std::borrow::Borrow;
use shrev::EventChannel;
use crate::event::{Event, WindowEvent, InputEvent, KeyboardInput};

use wasm_bindgen::prelude::*;
use wasm_bindgen::*;
use web_sys::*;
use web_sys::WebGl2RenderingContext as GL;

fn attach_keydown_callback(document: &Document) -> Result<(), JsValue> {
    let handler = move |event: web_sys::KeyboardEvent| {
        info!("Keydown: {}", event.key());
    };

    let handler = Closure::wrap(Box::new(handler) as Box<FnMut(_)>);
    document.add_event_listener_with_callback("keydown", handler.as_ref().unchecked_ref())?;
    handler.forget();

    info!("Attaching keydown handler to canvas");

    Ok(())
}


fn attach_keyup_callback(document: &Document) -> Result<(), JsValue> {
    let handler = move |event: web_sys::KeyboardEvent| {
        info!("Keyup: {}", event.key());
    };

    let handler = Closure::wrap(Box::new(handler) as Box<FnMut(_)>);
    document.add_event_listener_with_callback("keyup", handler.as_ref().unchecked_ref())?;
    handler.forget();

    info!("Attaching keyup handler to canvas");

    Ok(())
}

fn init_canvas() -> Result<HtmlCanvasElement, JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let canvas: HtmlCanvasElement = document.get_element_by_id("viewport").unwrap().dyn_into()?;
    canvas.set_width(320);
    canvas.set_height(240);
    canvas.style().set_property("display", "inline")?;
    canvas.style().set_property("float", "left")?;

//    attach_keydown_callback(&document)?;
//    attach_keyup_callback(&document)?;

    Ok(canvas)
}

pub fn create_webgl_context() -> Result<GL, JsValue> {
    let canvas = init_canvas()?;
    let gl: GL = canvas
        .get_context("webgl2")?
        .unwrap()
        .dyn_into::<GL>()?;
    gl.clear_color(0.2, 0.0, 0.0, 1.0);
    gl.enable(GL::DEPTH_TEST);
    Ok(gl)
}
