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
}

pub struct Checkbox {
    label: String,
    state: bool,
}

impl Slider {
    pub fn new(label: &str, min: f32, max: f32, step: f32, default: f32) -> Self {
        Slider { label: label.to_string(), min, max, step, default }
    }
}

impl Checkbox {
    pub fn new(label: &str, state: bool) -> Self {
        Checkbox { label: label.to_string(), state }
    }
}

pub trait Html {
    fn element(&self, app: Rc<App>, document: &Document) -> Result<HtmlElement, JsValue>;
}

impl Html for Slider {
    fn element(&self, app: Rc<App>, document: &Document) -> Result<HtmlElement, JsValue> {

        let form: HtmlFormElement = document.create_element("form")?.dyn_into()?;
        let label: HtmlLabelElement = document.create_element("label")?.dyn_into()?;
        let input: HtmlInputElement = document.create_element("input")?.dyn_into()?;
        let output: HtmlOutputElement = document.create_element("output")?.dyn_into()?;

        form.style().set_property("padding", "5px")?;
        form.style().set_property("font-size", "0")?;
        form.style().set_property("display", "flex")?;
        form.style().set_property("align-items", "center")?;
        label.style().set_property("font-size", "16")?;
        label.style().set_property("font-family", "sans-serif")?;
        output.style().set_property("font-size", "16")?;
        output.style().set_property("font-family", "sans-serif")?;
        label.set_inner_html(&format!("{}", self.label));
        input.set_type("range");
        input.set_min(&format!("{}", self.min));
        input.set_max(&format!("{}", self.max));
        input.set_step(&format!("{}", self.step));
        input.set_value(&format!("{}", self.default));
        output.set_value(&format!("{}", self.default));

        let key = self.label.clone();
        let callback = Closure::wrap(Box::new(move |event: web_sys::Event| {
            let input: HtmlInputElement = event.target().unwrap().dyn_into().unwrap();
            let value: f32 = input.value().parse().unwrap();
            let output: HtmlOutputElement = input.next_sibling().unwrap().dyn_into().unwrap();
            output.set_value(format!("{}", value).as_str());
            info!("Slider '{}': {}", key.clone(), value);
        }) as Box<FnMut(web_sys::Event)>);
        form.set_oninput(Some(&callback.as_ref().unchecked_ref()));
        callback.forget();

        form.append_child(&label)?;
        form.append_child(&input)?;
        form.append_child(&output)?;

        Ok(form.dyn_into::<HtmlElement>()?)
    }
}

impl Html for Checkbox {
    fn element(&self, app: Rc<App>, document: &Document) -> Result<HtmlElement, JsValue> {

        let form: HtmlFormElement = document.create_element("form")?.dyn_into()?;
        let label: HtmlLabelElement = document.create_element("label")?.dyn_into()?;
        let input: HtmlInputElement = document.create_element("input")?.dyn_into()?;

        form.style().set_property("padding", "5px")?;
        form.style().set_property("font-size", "0")?;
        form.style().set_property("display", "flex")?;
        form.style().set_property("align-items", "center")?;
        label.style().set_property("font-size", "16")?;
        label.style().set_property("font-family", "sans-serif")?;
        label.set_inner_html(&format!("{}", self.label));
        input.set_type("checkbox");
        input.set_value(&format!("{}", self.state as i32));

        let key: String = self.label.clone();
        let callback = Closure::wrap(Box::new(move |event: web_sys::Event| {
            let input: HtmlInputElement = event.target().unwrap().dyn_into().unwrap();
            let value: bool = input.checked();
            info!("Checkbox: {}", value);
        }) as Box<FnMut(web_sys::Event)>);
        form.set_oninput(Some(&callback.as_ref().unchecked_ref()));
        callback.forget();

        form.append_child(&input)?;
        form.append_child(&label)?;

        Ok(form.dyn_into::<HtmlElement>()?)
    }
}

pub fn append_controls(app: Rc<App>) -> Result<(), JsValue> {
    let window = window().unwrap();
    let document = window.document().unwrap();
    let ui: HtmlElement = document.get_element_by_id("ui").unwrap().dyn_into()?;
    ui.style().set_property("float", "left")?;

    let slider_x = Slider::new("X", 0.0, 100.0, 1.0, 55.0);
    let slider_y = Slider::new("Y", 0.0, 44.0, 1.0, 25.0);
    let check_a = Checkbox::new("Yes", false);

    ui.append_child(&slider_x.element(Rc::clone(&app), &document)?.dyn_into()?)?;
    ui.append_child(&slider_y.element(Rc::clone(&app), &document)?.dyn_into()?)?;
    ui.append_child(&check_a.element(Rc::clone(&app), &document)?.dyn_into()?)?;

    Ok(())
}