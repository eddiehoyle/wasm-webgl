#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate wasm_bindgen;
extern crate nalgebra_glm;

#[macro_use]
extern crate log;
extern crate wasm_logger;
extern crate specs;
extern crate shrev;

pub mod client;
mod render;
mod texture;
mod app;
mod shader;
mod prim;
