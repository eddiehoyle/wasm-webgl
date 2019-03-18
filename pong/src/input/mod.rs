use shrev::EventChannel;
use crate::event::*;
use std::collections::HashMap;
use std::collections::HashSet;

pub mod system;

#[derive(Default)]
pub struct InputHandler {
    keyset: HashSet<String>,
}

impl InputHandler {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn press(&mut self, key: &String) {
        self.keyset.insert(key.clone());
    }
    pub fn release(&mut self, key: &String) {
        self.keyset.remove(key);
    }
    pub fn is_pressed(&self, key: &String) -> bool {
        self.keyset.get(key).is_some()
    }
}