use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    window, CanvasRenderingContext2d, Document, Element, Event, HtmlCanvasElement,
    HtmlImageElement, HtmlInputElement,
};

use crate::state::State;

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

    // pen
    let pen = create_pen_element(&document, canvas)?;
    toolbar.append_child(&pen)?;

    // eraser
    let eraser = create_eraser_element(&document, canvas)?;
    toolbar.append_child(&eraser)?;

    // pen thin
    for thin in PEN_THIN.iter() {
        let pen_thin = create_pen_thin_element(*thin, &document, state)?;
        toolbar.append_child(&pen_thin)?;
    }

    // undo
    let undo = create_undo_element(&document, canvas, state)?;
    toolbar.append_child(&undo)?;

    // clear
    let clear = create_clear_element(&document, canvas, state)?;
    toolbar.append_child(&clear)?;

    // add preview
    let preview_image_list = create_preview_image_element(&document, canvas, preview, state)?;
    toolbar.append_child(&preview_image_list)?;

    Ok(())
}

static PEN_THIN: [f64; 5] = [1.0, 4.0, 8.0, 10.0, 15.0];

fn create_pen_thin_element(
    thin: f64,
    document: &Document,
    state: &Rc<RefCell<State>>,
) -> Result<Element, JsValue> {
    let element = document.create_element("div")?;

    element.set_attribute(
        "style",
        "height: 50px; width: 50px; display: flex; align-items: center; justify-content: center; font-size: 11px; border: 1px solid #9b9b9b;",
    )?;

    let inner_element = document.create_element("div")?;

    inner_element.set_attribute(
        "style",
        format!(
            "border-radius: 50%; background-color: black; width: {}px; height: {}px;",
            thin + 2.0,
            thin + 2.0
        )
        .as_str(),
    )?;
    element.append_child(&inner_element)?;

    let state = state.clone();

    let handle_click = Closure::wrap(Box::new(move || {
        state.borrow_mut().set_pen_thin(thin);
    }) as Box<dyn FnMut()>);

    element.add_event_listener_with_callback("click", handle_click.as_ref().unchecked_ref())?;

    handle_click.forget();

    Ok(element)
}

fn create_pen_element(document: &Document, canvas: &HtmlCanvasElement) -> Result<Element, JsValue> {
    let element = document.create_element("div")?;
    element.set_attribute(
        "style",
        "height: 50px; width: 50px; display: flex; align-items: center; justify-content: center; font-size: 11px; border: 1px solid #9b9b9b; background-image:url(https://image.flaticon.com/icons/svg/760/760400.svg); background-size: 100%;",
    )?;

    let context = canvas
        .get_context("2d")
        .expect("Could not get context")
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    let handle_click = Closure::wrap(Box::new(move || {
        context
            .set_global_composite_operation("source-over")
            .unwrap();
    }) as Box<dyn FnMut()>);

    element.add_event_listener_with_callback("click", handle_click.as_ref().unchecked_ref())?;

    handle_click.forget();

    Ok(element)
}

fn create_eraser_element(
    document: &Document,
    canvas: &HtmlCanvasElement,
) -> Result<Element, JsValue> {
    let element = document.create_element("div")?;
    element.set_attribute(
        "style",
        "height: 50px; width: 50px; display: flex; align-items: center; justify-content: center; font-size: 11px; border: 1px solid #9b9b9b; background-image:url(https://image.flaticon.com/icons/svg/200/200404.svg); background-size: 100%;",
    )?;

    let context = canvas
        .get_context("2d")
        .expect("Could not get context")
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    let handle_click = Closure::wrap(Box::new(move || {
        context
            .set_global_composite_operation("destination-out")
            .unwrap();
    }) as Box<dyn FnMut()>);

    element.add_event_listener_with_callback("click", handle_click.as_ref().unchecked_ref())?;

    handle_click.forget();

    Ok(element)
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
    element.set_attribute(
        "style",
        "height: 50px; width: 50px; display: flex; align-items: center; justify-content: center; font-size: 11px; border: 1px solid #9b9b9b; background-image:url(https://image.flaticon.com/icons/svg/318/318262.svg); background-size: 100%;",
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
    element.set_attribute(
        "style",
        "height: 50px; width: 50px; display: flex; align-items: center; justify-content: center; font-size: 11px; border: 1px solid #9b9b9b; background-image:url(https://image.flaticon.com/icons/svg/1562/1562881.svg); background-size: 100%;",
    )?;

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

        let url = canvas.to_data_url_with_type("image/png").unwrap();
        state.borrow_mut().add_preview_image(url.clone());

        // img set_src URL string
        img.set_src(&url);
        img.set_attribute("class", "preview-img").unwrap();
        img.set_width(state.borrow().get_preview_width());
        img.set_height(state.borrow().get_preview_height());
        preview.append_child(&img).unwrap();
    }) as Box<dyn FnMut()>);
    element.add_event_listener_with_callback("click", handle_click.as_ref().unchecked_ref())?;
    handle_click.forget();

    Ok(element)
}
