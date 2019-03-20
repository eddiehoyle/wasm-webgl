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
use     std::rc::Rc;

// -----------

// -----------
pub struct RenderSystem {
    gl: GL,
}

impl RenderSystem {
    pub fn new(canvas: HtmlCanvasElement) -> Self {
        let gl: GL = canvas
            .get_context("webgl2").unwrap()
            .unwrap()
            .dyn_into::<GL>().unwrap();
        RenderSystem {gl}
    }
}


impl<'a> System<'a> for RenderSystem {
    type SystemData = ();

    fn run(&mut self, _: Self::SystemData) {
        self.gl.clear_color(0.4, 0.0, 0.0, 1.0);
        self.gl.clear(GL::COLOR_BUFFER_BIT);
        self.gl.enable(GL::DEPTH_TEST);
        self.gl.depth_func(GL::LEQUAL);
    }

    fn setup(&mut self, res: &mut Resources) {
        use specs::prelude::SystemData;
        Self::SystemData::setup(res);
        info!("Setting up RenderSystem");
    }
}