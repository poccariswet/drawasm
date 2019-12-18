use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, Element, HtmlCanvasElement, HtmlElement};

mod draw;
mod state;
mod utils;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn initialize() {
    utils::set_panic_hook();
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(msg: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    let document = window()
        .unwrap()
        .document()
        .expect("Could not find `document`");

    let body = document.body().expect("Could not find `body` element");
    let toolbar = document.get_element_by_id("toolbar").unwrap();
    let preview = document.get_element_by_id("preview").unwrap();
    preview.set_attribute("style", "background-color:#d0d7db;")?;
    preview.set_inner_html("preview");
    let canvas = document.get_element_by_id("draw").unwrap();

    // set canvas, preview dimention
    let (w, h) = get_body_dimensions(&body);
    let (_, bar_h) = get_el_dimensions(&toolbar);
    let (pre_w, _) = get_el_dimensions(&preview);
    let canvas: HtmlCanvasElement = canvas
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    canvas.style().set_property("border", "2px solid")?;

    let canvas_w = w - (pre_w + 4);
    let canvas_h = h - (bar_h + 4);

    canvas.set_width(canvas_w);
    canvas.set_height(canvas_h);

    let state: Rc<RefCell<state::State>> =
        Rc::new(RefCell::new(state::State::new(canvas_w, canvas_h)));

    draw::canvas_draw_start(&canvas, &state)?;

    Ok(())
}

fn get_el_dimensions(el: &Element) -> (u32, u32) {
    let width = el.client_width() as u32;
    let height = el.client_height() as u32;

    (width, height)
}
fn get_body_dimensions(body: &HtmlElement) -> (u32, u32) {
    let width = body.client_width() as u32;
    let height = body.client_height() as u32;

    (width, height)
}
