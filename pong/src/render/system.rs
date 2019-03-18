use shrev::{EventChannel, ReaderId};
use specs::prelude::{Read, Resources, System, Write};
use crate::event::*;
use crate::input::InputHandler;
use crate::client::dom::create_webgl_context;

use wasm_bindgen::prelude::*;
use wasm_bindgen::*;
use web_sys::*;
use web_sys::WebGl2RenderingContext as GL;
use specs::Component;
use specs::VecStorage;
use specs::{ReadStorage};
use specs::prelude::*;
use shred_derive::*;
use std::rc::Rc;

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

#[derive(Default)]
struct WebGLRender {
//    gl: GL,
}

// -----------

pub struct RenderSystem {
    render: WebGLRender,
}

impl RenderSystem {

    pub fn new() -> Self {
        RenderSystem {render: WebGLRender::default()}
    }
}


impl<'a> System<'a> for RenderSystem {
    type SystemData = Renderable<'a>;

    fn run(&mut self, input: Self::SystemData) {
//        info!("Rendering!")
    }

    fn setup(&mut self, res: &mut Resources) {
        use specs::prelude::SystemData;
        Self::SystemData::setup(res);
        self.render = WebGLRender {};

//        self.reader = Some(res.fetch_mut::<GL>().register_reader());

        info!("Setting up RenderSystem");
    }
}