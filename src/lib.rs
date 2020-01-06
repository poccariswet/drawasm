use std::cell::RefCell;
use std::cmp::{max, min};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, Element, HtmlCanvasElement, HtmlElement};

mod draw;
mod generate;
mod state;
mod toolbar;
mod utils;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn initialize() {
    utils::set_panic_hook();
}

static TOOLBAR_HEIGHT: u32 = 50;

#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    let document = window()
        .unwrap()
        .document()
        .expect("Could not find `document`");

    let body = document.body().expect("Could not find `body` element");
    let toolbar = document.get_element_by_id("toolbar").unwrap();
    let preview = document.get_element_by_id("preview").unwrap();
    let canvas = document.get_element_by_id("draw").unwrap();

    // set canvas, preview dimention
    let (w, h) = get_body_dimensions(&body);
    let (pre_w, _) = get_el_dimensions(&preview);
    let canvas: HtmlCanvasElement = canvas
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let canvas_w = w - (pre_w + 5);
    let canvas_h = h - (TOOLBAR_HEIGHT + 100 + 5);

    canvas.set_width(canvas_w);
    canvas.set_height(canvas_h);

    let preview = document.get_element_by_id("preview").unwrap();
    let (pre_w, pre_h) = get_el_dimensions(&preview);
    preview.set_attribute(
        "style",
        format!("width: {}px; height: {}px;", pre_w, pre_h).as_str(),
    )?;

    toolbar.set_attribute("style", format!("width: {}px;", canvas_w + 5).as_str())?;

    let state: Rc<RefCell<state::State>> =
        Rc::new(RefCell::new(state::State::new(canvas_w, canvas_h)));

    draw::canvas_draw_start(&canvas, &state)?;
    toolbar::init_toolbar(&toolbar, &canvas, &preview, &state)?;
    generate::init_generate(&state)?;

    Ok(())
}

fn get_el_dimensions(el: &Element) -> (u32, u32) {
    let width = min(max(el.client_width() as u32, 250), 1500);
    let height = min(max(el.client_height() as u32, 450), 1800);

    (width, height)
}
fn get_body_dimensions(body: &HtmlElement) -> (u32, u32) {
    let width = min(max(body.client_width() as u32, 600), 3000);
    let height = min(max(body.client_height() as u32, 400), 2000);

    (width, height)
}
