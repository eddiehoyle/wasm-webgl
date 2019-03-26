#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_variables)]

#[macro_use]
extern crate log;
extern crate wasm_bindgen;
extern crate wasm_logger;
extern crate specs;
extern crate shrev;
extern crate cgmath;

pub mod client;
mod app;
mod event;
mod input;
mod render;