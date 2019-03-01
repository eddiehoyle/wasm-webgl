use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::window;
use web_sys::Element;
use web_sys::Document;
use web_sys::HtmlElement;
use web_sys::HtmlLabelElement;
use web_sys::HtmlFormElement;
use web_sys::HtmlInputElement;
use web_sys::HtmlOutputElement;
use crate::app::App;


pub struct Slider {
    label: String,
    min: f32,
    max: f32,
    step: f32,
    default: f32,
    callback: Closure<FnMut(web_sys::Event)>,
}

impl Slider {
    pub fn new(label: &str, min: f32, max: f32, step: f32, default: f32, callback: Closure<FnMut(web_sys::Event)>) -> Self {
        Slider { label: label.to_string(), min, max, step, default, callback }
    }
}

pub trait Html {
    fn element(&self, document: &Document) -> Result<HtmlElement, JsValue>;
}

impl Html for Slider {
    fn element(&self, document: &Document) -> Result<HtmlElement, JsValue> {

        let form: HtmlFormElement = document.create_element("form")?.dyn_into()?;
        let label: HtmlLabelElement = document.create_element("label")?.dyn_into()?;
        let input: HtmlInputElement = document.create_element("input")?.dyn_into()?;
        let output: HtmlOutputElement = document.create_element("output")?.dyn_into()?;

        form.style().set_property("padding", "5px")?;
        form.style().set_property("font-size", "0")?;
        label.style().set_property("font-size", "16")?;
        output.style().set_property("font-size", "16")?;
        label.set_inner_html("X");
        input.set_type("range");
        input.set_min(&format!("{}", self.min));
        input.set_max(&format!("{}", self.max));
        input.set_step(&format!("{}", self.step));
        input.set_value(&format!("{}", self.default));
        output.set_value(&format!("{}", self.default));

        let oninput = move |event: web_sys::Event| {
            let input: HtmlInputElement = event.target().unwrap().dyn_into().unwrap();
            let value : f32 = input.value().parse().unwrap();
            info!("Value: {}", value);
            let output: HtmlOutputElement = input.next_sibling().unwrap().dyn_into().unwrap();
            output.set_value(format!("{}", value).as_str());
        };
        let closure = Closure::wrap(Box::new(oninput) as Box<FnMut(_)>);
        form.set_oninput(Some(&closure.as_ref().unchecked_ref()));
        closure.forget();
//        form.set_oninput(Some(&self.closure.as_ref().unchecked_ref()));

//        parent.append_child(&form)?;
        form.append_child(&label)?;
        form.append_child(&input)?;
        form.append_child(&output)?;

        Ok(form.dyn_into::<HtmlElement>()?)
    }
}

pub fn create_control<T>(obj: &T, document: &Document) -> Result<HtmlElement, JsValue>
where
    T: Html {
    obj.element(document)
}

pub fn append_controls(app: Rc<App>) -> Result<(), JsValue> {
    let window = window().unwrap();
    let document = window.document().unwrap();
    let ui: HtmlElement = document.get_element_by_id("ui").unwrap().dyn_into()?;

    let callback_x = Closure::wrap(Box::new(move |event: web_sys::Event| {
        let input: HtmlInputElement = event.target().unwrap().dyn_into().unwrap();
        let value: f32 = input.value().parse().unwrap();
        let output: HtmlOutputElement = input.next_sibling().unwrap().dyn_into().unwrap();
        output.set_value(format!("{}", value).as_str());
    }) as Box<FnMut(web_sys::Event)>);

    let slider_x = Slider::new("X", 0.0, 100.0, 1.0, 55.0, callback_x);

    let callback_y = Closure::wrap(Box::new(move |event: web_sys::Event| {
        let input: HtmlInputElement = event.target().unwrap().dyn_into().unwrap();
        let value: f32 = input.value().parse().unwrap();
        let output: HtmlOutputElement = input.next_sibling().unwrap().dyn_into().unwrap();
        output.set_value(format!("{}", value).as_str());
    }) as Box<FnMut(web_sys::Event)>);
    let slider_y = Slider::new("Y", 0.0, 25.0, 1.0, 2.0, callback_y);

    let x = create_control(&slider_x, &document)?;
    let y = create_control(&slider_y, &document)?;

    ui.append_child(&x)?;
    ui.append_child(&y)?;

    Ok(())
}