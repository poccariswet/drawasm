use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    window, CanvasRenderingContext2d, Document, Element, Event, HtmlCanvasElement, HtmlInputElement,
};

use crate::state::State;

pub fn init_toolbar(
    toolbar: &Element,
    canvas: &HtmlCanvasElement,
    state: &Rc<RefCell<State>>,
) -> Result<(), JsValue> {
    let document = window()
        .unwrap()
        .document()
        .expect("Could not find `document`");

    // color picker
    let color_pick = create_color_picker(&document, state)?;
    toolbar.append_child(&color_pick)?;

    // undo
    let undo = create_undo_element(&document, canvas, state)?;
    toolbar.append_child(&undo)?;

    Ok(())
}

fn create_color_picker(
    document: &Document,
    state: &Rc<RefCell<State>>,
) -> Result<Element, JsValue> {
    let element = document.create_element("div")?;
    element.set_attribute(
        "style",
        "height: 50px; width: 50px; display: flex; align-items: center; justify-content: center; font-size: 11px; border: 1px solid #9b9b9b;",
    )?;

    let input = document
        .create_element("input")?
        .dyn_into::<HtmlInputElement>()?;

    input.set_attribute("type", "color")?;
    input.set_attribute("value", "#000000")?;

    let state = state.clone();
    let picked_color = Closure::wrap(Box::new(move |e: Event| {
        let target = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
        let color = target.value();
        state.borrow_mut().set_color(color)
    }) as Box<dyn FnMut(_)>);
    input.add_event_listener_with_callback("change", picked_color.as_ref().unchecked_ref())?;
    picked_color.forget();
    element.append_child(&input)?;

    Ok(element)
}

fn create_undo_element(
    document: &Document,
    canvas: &HtmlCanvasElement,
    state: &Rc<RefCell<State>>,
) -> Result<Element, JsValue> {
    let element = document.create_element("div")?;
    element.set_inner_html("Undo");
    element.set_attribute(
        "style",
        "height: 50px; width: 50px; display: flex; align-items: center; justify-content: center; font-size: 11px; border: 1px solid #9b9b9b;",
    )?;

    let context = canvas
        .get_context("2d")
        .expect("Could not get context")
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();
    let state = state.clone();

    let handle_click = Closure::wrap(Box::new(move || {
        let undo = state.borrow_mut().get_undo();
        match undo {
            Some(u) => {
                context.put_image_data(&u, 0.0, 0.0).unwrap();
            }
            None => {}
        }
    }) as Box<dyn FnMut()>);
    element.add_event_listener_with_callback("click", handle_click.as_ref().unchecked_ref())?;
    handle_click.forget();

    Ok(element)
}
