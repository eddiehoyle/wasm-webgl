use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::window;
use web_sys::Element;
use web_sys::HtmlElement;
use web_sys::HtmlLabelElement;
use web_sys::HtmlFormElement;
use web_sys::HtmlInputElement;
use crate::app::App;


pub fn append_controls(app: &App) -> Result<(), JsValue> {
    let window = window().unwrap();
    let document = window.document().unwrap();

    let container: HtmlElement = match document.get_element_by_id("simple_app") {
        Some(container) => container.dyn_into().expect("Html element"),
        None => document.body().expect("Document body"),
    };

    let controls = document.create_element("div")?;
    container.append_child(&controls)?;
    let controls: HtmlElement = controls.dyn_into()?;
    controls.style().set_property("padding-left", "5px")?;
    let controls: Element = controls.dyn_into()?;

    info!("Creating controls");

    // Wave Speed
    {
//        let app = Rc::clone(&app);
//        let wave_speed_control = create_wave_speed_control(app)?;
        let control = create_control(app, "sdf", 0.0, 12.0, 0.2)?;
        controls.append_child(&control)?;
    }

    Ok(())
}

fn create_control(app: &App, label: &'static str, min: f32, max: f32, step: f32) -> Result<HtmlElement, JsValue> {
    let handler = move |event: web_sys::Event| {
        let input_elem: HtmlInputElement = event.target().unwrap().dyn_into().unwrap();
        let value : f32 = input_elem.value().parse().unwrap();
    };
    let closure = Closure::wrap(Box::new(handler) as Box<FnMut(_)>);

    let control = Slider {
        min,
        max,
        step,
        start: 0.0,
        label: label.clone(),
        closure,
    }.create_element()?;

    Ok(control)
}


fn create_wave_speed_control(app: &App) -> Result<HtmlElement, JsValue> {
//    let handler = move |event: web_sys::Event| {
//        let input_elem: HtmlInputElement = event.target().unwrap().dyn_into().unwrap();
//        let wave_speed : f32 = input_elem.value().parse().unwrap();
////        app.store.borrow_mut().msg(&Msg::SetWaveSpeed(wave_speed));
//    };
//    let closure = Closure::wrap(Box::new(handler) as Box<FnMut(_)>);
    let closure = Closure::wrap(Box::new(move |event: web_sys::Event|{}) as Box<FnMut(_)>);

    info!("Creating wave speed");

    let wave_speed_control = Slider {
        min: 0.0,
        max: 0.15,
        step: 0.01,
        start: 0.06,
        label: "Wave Speed",
        closure,
    }.create_element()?;

    Ok(wave_speed_control)
}

struct Slider {
    min: f32,
    max: f32,
    step: f32,
    start: f32,
    label: &'static str,
    closure: Closure<FnMut(web_sys::Event)>,
}

impl Slider {
    fn create_element(self) -> Result<HtmlElement, JsValue> {

        info!("Creating slider element");

        let window = window().unwrap();
        let document = window.document().unwrap();

        let label: HtmlLabelElement = document.create_element("label")?.dyn_into()?;
        label.set_inner_html(format!("{}:", self.label).as_str());

        let slider: HtmlInputElement = document.create_element("input")?.dyn_into()?;
        slider.set_type("range");
        slider.set_min(&format!("{}", self.min));
        slider.set_max(&format!("{}", self.max));
        slider.set_step(&format!("{}", self.step));
        slider.set_value(&format!("{}", self.start));

//        let closure = self.closure;
//        slider.set_oninput(Some(closure.as_ref().unchecked_ref()));
//        closure.forget();

        let value = document.create_element("output")?;

        let oninput = move |event: web_sys::Event| {
            let elem: HtmlInputElement = event.target().unwrap().dyn_into().unwrap();
            let v : f32 = elem.value().parse().unwrap();
            let elem: HtmlElement = elem.dyn_into().unwrap();
//            for child in elem.children() {
//
//            }

//            info!("input: {}", v);
//            value.set_inner_html(v.to_string().as_str());

        };
        let closure = Closure::wrap(Box::new(oninput) as Box<FnMut(_)>);

        let container: HtmlFormElement = document.create_element("form")?.dyn_into()?;
        container.set_oninput(Some(&closure.as_ref().unchecked_ref()));
        closure.forget();

        container.append_child(&label)?;
        container.append_child(&slider)?;
        container.append_child(&value)?;

        let container: HtmlElement = container.dyn_into()?;
        container.style().set_property("margin-bottom", "15px")?;

        Ok(container)
    }
}