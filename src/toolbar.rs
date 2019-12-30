use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    window, Blob, BlobPropertyBag, CanvasRenderingContext2d, Document, Element, Event,
    HtmlCanvasElement, HtmlImageElement, HtmlInputElement, Url,
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

pub fn init_toolbar(
    toolbar: &Element,
    canvas: &HtmlCanvasElement,
    preview: &Element,
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

    // clear
    let clear = create_clear_element(&document, canvas, state)?;
    toolbar.append_child(&clear)?;

    // preview image list
    let preview_image_list = create_preview_image_element(&document, canvas, preview, state)?;
    toolbar.append_child(&preview_image_list)?;

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

fn create_clear_element(
    document: &Document,
    canvas: &HtmlCanvasElement,
    state: &Rc<RefCell<State>>,
) -> Result<Element, JsValue> {
    let element = document.create_element("div")?;
    element.set_inner_html("Clear");
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
        let image_data = context
            .get_image_data(
                0.0,
                0.0,
                state.borrow().get_width() as f64,
                state.borrow().get_height() as f64,
            )
            .unwrap();
        state.borrow_mut().add_undo(image_data);
        context.clear_rect(
            0.0,
            0.0,
            state.borrow().get_width() as f64,
            state.borrow().get_height() as f64,
        );
    }) as Box<dyn FnMut()>);
    element.add_event_listener_with_callback("click", handle_click.as_ref().unchecked_ref())?;
    handle_click.forget();

    Ok(element)
}

fn create_preview_image_element(
    document: &Document,
    canvas: &HtmlCanvasElement,
    preview: &Element,
    state: &Rc<RefCell<State>>,
) -> Result<Element, JsValue> {
    let element = document.create_element("div")?;
    element.set_inner_html("add");
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

    let canvas = canvas.clone();
    let state = state.clone();
    let preview = preview.clone();
    let document_copy = document.clone();

    let handle_click = Closure::wrap(Box::new(move || {
        let img = document_copy
            .create_element("img")
            .unwrap()
            .dyn_into::<HtmlImageElement>()
            .unwrap();

        let image_data = context
            .get_image_data(
                0.0,
                0.0,
                state.borrow().get_width() as f64,
                state.borrow().get_height() as f64,
            )
            .unwrap();

        let buffer = image_data.data().to_vec(); // Vec<u8> image data
        state.borrow_mut().add_preview_image(buffer);

        //let mut blob_property = BlobPropertyBag::new();
        //let array = js_sys::Uint8Array::from(buffer.as_slice());
        //let blob =
        //    Blob::new_with_u8_array_sequence_and_options(&array, blob_property.type_("image/png"))
        //        .unwrap();
        //console_log!("{:?}", blob);
        //let url = Url::create_object_url_with_blob(&blob).unwrap();
        //console_log!("{}", url);

        let url = canvas.to_data_url_with_type("image/png").unwrap();

        // img set_src URL string
        img.set_src(&url);
        img.set_attribute("class", "preview-img");
        img.set_width(state.borrow().get_preview_width());
        img.set_height(state.borrow().get_preview_height());
        preview.append_child(&img);
    }) as Box<dyn FnMut()>);
    element.add_event_listener_with_callback("click", handle_click.as_ref().unchecked_ref())?;
    handle_click.forget();

    Ok(element)
}
