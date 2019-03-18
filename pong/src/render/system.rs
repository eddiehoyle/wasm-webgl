use shrev::{EventChannel, ReaderId};
use specs::prelude::{Read, Resources, System, Write};
use crate::event::*;
use crate::input::InputHandler;

use wasm_bindgen::prelude::*;
use wasm_bindgen::*;
use web_sys::*;
use web_sys::WebGl2RenderingContext as GL;
use specs::Component;
use specs::VecStorage;
use specs::{ReadStorage};
use specs::prelude::*;
use shred_derive::*;

// -----------

#[derive(Clone, Debug)]
pub struct Pos {}

impl Component for Pos {
    type Storage = VecStorage<Self>;
}

#[derive(SystemData)]
pub struct Renderable<'a> {
    pos: ReadStorage<'a, Pos>,
}

// -----------

pub struct RenderSystem {
}

impl RenderSystem {

    pub fn new() -> Self {
        RenderSystem {}
    }
}


impl<'a> System<'a> for RenderSystem {
    type SystemData = Renderable<'a>;

    fn run(&mut self, input: Self::SystemData) {
        info!("Rendering!")
    }

    fn setup(&mut self, res: &mut Resources) {
        use specs::prelude::SystemData;
        Self::SystemData::setup(res);
        info!("Setting up RenderSystem");
    }
}