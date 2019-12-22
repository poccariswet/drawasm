use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    window, CanvasRenderingContext2d, Document, Element, Event, HtmlCanvasElement, HtmlInputElement,
};

use crate::state::State;

pub fn init_generate(preview: &Element, state: &Rc<RefCell<State>>) -> Result<(), JsValue> {
    Ok(())
}

fn create_generate_button() -> Result<(), JsValue> {
    Ok(())
}
