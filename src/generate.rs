use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    window, CanvasRenderingContext2d, Document, Element, Event, HtmlButtonElement,
    HtmlCanvasElement, HtmlInputElement,
};

use crate::state::State;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(msg: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

pub fn init_generate(state: &Rc<RefCell<State>>) -> Result<(), JsValue> {
    let document = window()
        .unwrap()
        .document()
        .expect("Could not find `document`");

    let generate = document.get_element_by_id("generate").unwrap();

    let button = create_generate_button(&document, state)?;
    generate.append_child(&button)?;
    Ok(())
}

fn create_generate_button(
    document: &Document,
    state: &Rc<RefCell<State>>,
) -> Result<HtmlButtonElement, JsValue> {
    let button = document
        .create_element("button")?
        .dyn_into::<HtmlButtonElement>()?;
    button.set_attribute("class", "btn")?;
    button.set_inner_html("APNG generate");

    let handle_click = Closure::wrap(Box::new(sample) as Box<dyn Fn()>);
    button.set_onclick(Some(handle_click.as_ref().unchecked_ref()));
    handle_click.forget();

    Ok(button)
}

fn sample() {
    console_log!("clicked!!!");
}
