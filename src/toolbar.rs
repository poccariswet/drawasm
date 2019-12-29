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

    let img = document
        .create_element("img")?
        .dyn_into::<HtmlImageElement>()?;

    let handle_click = Closure::wrap(Box::new(move || {
        let image_data = context
            .get_image_data(
                0.0,
                0.0,
                state.borrow().get_width() as f64,
                state.borrow().get_height() as f64,
            )
            .unwrap();

        //let buffer = wasm.apngEncodeAll(buffers, frame_speed);
        //var blob = new Blob([buffer], {type: 'image/png'});
        //var url = window.URL.createObjectURL(blob);
        //var elem = document.getElementById("apng");
        //elem.src = url;

        let buffer = image_data.data().to_vec(); // Vec<u8> image data
                                                 //console_log!("{:?}", buffer);

        let mut blob_property = BlobPropertyBag::new();
        let array = js_sys::Uint8Array::from(buffer.as_slice());
        //  Blob new_with_u8_array_sequence_and_options
        let blob =
            Blob::new_with_u8_array_sequence_and_options(&array, blob_property.type_("image/png"))
                .unwrap();
        console_log!("{:?}", blob);
        //  URL create_object_url_with_blob -> String
        let url = Url::create_object_url_with_blob(&blob).unwrap();
        console_log!("{}", url);
        let window = web_sys::window().expect("no global `window` exists");
        //window.open_with_url(&url);

        // img set_src URL string
        img.set_src(&url);
        preview.append_child(&img);

        state.borrow_mut().add_preview_image(buffer);
    }) as Box<dyn FnMut()>);
    element.add_event_listener_with_callback("click", handle_click.as_ref().unchecked_ref())?;
    handle_click.forget();

    Ok(element)
}
