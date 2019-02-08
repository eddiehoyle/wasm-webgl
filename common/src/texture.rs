//use crate::render::TextureUnit;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlImageElement;
use web_sys::WebGl2RenderingContext;

pub fn load_texture_image(context: &WebGl2RenderingContext, src: &str) {
//    let image = Rc::new(RefCell::new(HtmlImageElement::new().unwrap()));
//    let image_clone = Rc::clone(&image);
    let image = HtmlImageElement::new().unwrap();

    let texture = context.create_texture();
    context.active_texture(WebGl2RenderingContext::TEXTURE0);
    context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, texture.as_ref());
    context.pixel_storei(WebGl2RenderingContext::UNPACK_FLIP_Y_WEBGL, 1);
    context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MIN_FILTER, WebGl2RenderingContext::NEAREST as i32);
    context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MAG_FILTER, WebGl2RenderingContext::NEAREST as i32);
    context.tex_image_2d_with_u32_and_u32_and_html_image_element(
        WebGl2RenderingContext::TEXTURE_2D,
        0,
        WebGl2RenderingContext::RGBA as i32,
        WebGl2RenderingContext::RGBA,
        WebGl2RenderingContext::UNSIGNED_BYTE,
        &image,
    ).expect("Texture image 2d");

//    image.set_onload(Some(onload.as_ref().unchecked_ref()));

//    onload.forget();


//    let image_clone = Rc::clone(&image);
//
//    let onload = Closure::wrap(Box::new(move || {
//        let texture = context.create_texture();
//        context.active_texture(WebGl2RenderingContext::TEXTURE0);
//        context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, texture.as_ref());
//        context.pixel_storei(WebGl2RenderingContext::UNPACK_FLIP_Y_WEBGL, 1);
//        context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MIN_FILTER, WebGl2RenderingContext::NEAREST as i32);
//        context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MAG_FILTER, WebGl2RenderingContext::NEAREST as i32);
//        context.tex_image_2d_with_u32_and_u32_and_html_image_element(
//            WebGl2RenderingContext::TEXTURE_2D,
//            0,
//            WebGl2RenderingContext::RGBA as i32,
//            WebGl2RenderingContext::RGBA,
//            WebGl2RenderingContext::UNSIGNED_BYTE,
//            &image_clone.borrow(),
//        ).expect("Texture image 2d");
//    }) as Box<dyn Fn()>);
//
//    let image = image.borrow_mut();
//
//    image.set_onload(Some(onload.as_ref().unchecked_ref()));
//    image.set_src(src);
//
//    onload.forget();
}
