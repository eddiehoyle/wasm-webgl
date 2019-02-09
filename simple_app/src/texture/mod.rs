use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::*;
use web_sys::WebGl2RenderingContext as GL;
use std::borrow::Borrow;

pub struct Texture {
    unit: u32,
}

impl Texture {
    pub fn new(gl: &Rc<GL>, path: &str, unit: u32) -> Self {
        let image = Rc::new(HtmlImageElement::new().unwrap());
        let onload_cb = Closure::wrap( onload(gl.clone(),
                                              image.clone(),
                                              unit));
        let image : &HtmlImageElement = image.borrow();
        image.set_src(path);
        image.set_onload(Some(onload_cb.as_ref().unchecked_ref()));
        onload_cb.forget();
        Texture { unit }
    }
}

fn onload(gl: Rc<GL>, image: Rc<HtmlImageElement>, unit: u32) -> Box<Fn()> {
    Box::new(move || {
        let texture = gl.create_texture();
        gl.active_texture(GL::TEXTURE0 + unit);
        gl.bind_texture(GL::TEXTURE_2D, texture.as_ref());
        gl.pixel_storei(GL::UNPACK_FLIP_Y_WEBGL, 1);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::NEAREST as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::NEAREST as i32);
        gl.tex_image_2d_with_u32_and_u32_and_html_image_element(
            GL::TEXTURE_2D,
            0,
            GL::RGBA as i32,
            GL::RGBA,
            GL::UNSIGNED_BYTE,
            &image.borrow(),
        ).expect("Texture image 2d");
        console::log_1(&JsValue::from("image loaded!"));
    })
}
