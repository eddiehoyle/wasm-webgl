#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate wasm_bindgen;

#[macro_use]
extern crate log;
extern crate wasm_logger;

pub mod client;
mod render;
mod texture;
mod app;
mod shader;
mod prim;
