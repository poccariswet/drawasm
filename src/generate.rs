use apng;
use apng::Encoder;
use apng::{Frame, PNGImage};
use std::cell::RefCell;
use std::io::BufWriter;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    window, Blob, BlobPropertyBag, CanvasRenderingContext2d, Document, Element, Event,
    HtmlButtonElement, HtmlCanvasElement, HtmlInputElement, Url,
};

use crate::state::State;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(msg: &str);
    fn alert(s: &str);
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

    let state = state.clone();
    let handle_click = Closure::wrap(Box::new(move || {
        if state.borrow().get_preview_image_len() == 0 {
            alert("not added image");
            return;
        }

        let preview_images = state.borrow().get_preview_image();
        let frame_speed = 0.33; //TODO: not hardcode

        let mut png_images: Vec<PNGImage> = Vec::new();
        for data in preview_images {
            let v = data.to_string().replace("data:image/png;base64,", "");

            let buffer = base64::decode(&v).unwrap();
            let img =
                image::load_from_memory_with_format(&buffer, image::ImageFormat::PNG).unwrap();
            png_images.push(apng::load_dynamic_image(img).unwrap());
        }

        let mut buf = Vec::new();
        {
            let mut buf_writer = BufWriter::new(&mut buf);

            let config = apng::create_config(&png_images, None).unwrap();
            let mut encoder = Encoder::new(&mut buf_writer, config).unwrap();
            let d_num = frame_speed * (100 as f64);
            let d_den = 100;

            let frame = Frame {
                delay_num: Some(d_num as u16),
                delay_den: Some(d_den),
                ..Default::default()
            };

            match encoder.encode_all(png_images, Some(&frame)) {
                Ok(_n) => log("success apng encode!!!"),
                Err(err) => console_log!("{}", err),
            }
        }

        let b = js_sys::Uint8Array::new(&unsafe { js_sys::Uint8Array::view(&buf) }.into());
        let array = js_sys::Array::new();
        array.push(&b.buffer());
        let blob = Blob::new_with_u8_array_sequence_and_options(
            &array,
            BlobPropertyBag::new().type_("image/png"),
        )
        .unwrap();
        let url = Url::create_object_url_with_blob(&blob).unwrap();
        let window = window().unwrap();
        window.open_with_url(&url);
    }) as Box<dyn FnMut()>);

    button.set_onclick(Some(handle_click.as_ref().unchecked_ref()));
    handle_click.forget();

    Ok(button)
}
