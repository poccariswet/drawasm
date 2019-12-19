use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{window, Element, HtmlCanvasElement};

use crate::state::State;

pub fn init_toolbar(
    toolbar: &Element,
    canvas: &HtmlCanvasElement,
    state: &Rc<RefCell<State>>,
) -> Result<(), JsValue> {
    Ok(())
}
