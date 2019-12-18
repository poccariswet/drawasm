use std::cell::Cell;
use std::cell::RefCell;
use std::f64;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent};

use crate::state::State;

// sample draw canvas
//#[warn(unused_variables)]
//fn draw_face() {
//    let document = web_sys::window().unwrap().document().unwrap();
//    let canvas = document.get_element_by_id("canvas").unwrap();
//    let canvas: web_sys::HtmlCanvasElement = canvas
//        .dyn_into::<web_sys::HtmlCanvasElement>()
//        .map_err(|_| ())
//        .unwrap();
//
//    let context = canvas
//        .get_context("2d")
//        .unwrap()
//        .unwrap()
//        .dyn_into::<web_sys::CanvasRenderingContext2d>()
//        .unwrap();
//
//    context.begin_path();
//
//    // Draw the face
//    context
//        .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
//        .unwrap();
//
//    // Draw the mouth.
//    context.move_to(110.0, 75.0);
//    context.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI).unwrap();
//
//    // Draw the left eye.
//    context.move_to(65.0, 65.0);
//    context.arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI).unwrap();
//
//    // Draw the right eye.
//    context.move_to(95.0, 65.0);
//    context
//        .arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
//        .unwrap();
//
//    context.stroke();
//}

/*
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(msg: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}
*/

// setup mouse event listener for drawing and start
pub fn canvas_draw_start(
    canvas: &HtmlCanvasElement,
    state: &Rc<RefCell<State>>,
) -> Result<(), JsValue> {
    let context = canvas
        .get_context("2d")
        .expect("Could not get context")
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();
    let pressed = Rc::new(Cell::new(false));

    // mousedown
    {
        let context_copy = context.clone();
        let canvas_copy = canvas.clone();
        let state_copy = state.clone();
        let pressed = pressed.clone();

        let mouse_down = Closure::wrap(Box::new(move |event: MouseEvent| {
            let undo = state_copy.borrow_mut().get_undo();
            match undo {
                Some(u) => {
                    context_copy.put_image_data(&u, 0.0, 0.0);
                }
                None => {}
            }
            pressed.set(true);
            let image_data = context_copy
                .get_image_data(
                    0.0,
                    0.0,
                    state_copy.borrow().get_width() as f64,
                    state_copy.borrow().get_height() as f64,
                )
                .unwrap();
            console_log!("{:?}", image_data);
            state_copy.borrow_mut().add_undo(image_data);
            //console_log!("{}", canvas_copy.to_data_url().unwrap());

            let new_x = event.offset_x() as f64;
            let new_y = event.offset_y() as f64;
            context_copy.begin_path();
            context_copy.set_stroke_style(&JsValue::from(state_copy.borrow().get_color()));
            context_copy.set_line_width(state_copy.borrow().get_pen_thin());
            context_copy.move_to(new_x, new_y);
        }) as Box<dyn FnMut(_)>);

        canvas
            .add_event_listener_with_callback("mousedown", mouse_down.as_ref().unchecked_ref())?;

        mouse_down.forget(); // memory leak
    }

    // mouseup
    {
        let context_copy = context.clone();
        let pressed = pressed.clone();

        let mouse_up = Closure::wrap(Box::new(move |event: MouseEvent| {
            pressed.set(false);
            let new_x = event.offset_x() as f64;
            let new_y = event.offset_y() as f64;
            context_copy.fill_rect(new_x, new_y, 1.0, 1.0);
            context_copy.line_to(new_x, new_y);
            context_copy.stroke();
        }) as Box<dyn FnMut(_)>);

        canvas.add_event_listener_with_callback("mouseup", mouse_up.as_ref().unchecked_ref())?;

        mouse_up.forget();
    }

    // mousemove
    {
        let context_copy = context.clone();
        let pressed = pressed.clone();

        let mouse_move = Closure::wrap(Box::new(move |event: MouseEvent| {
            if pressed.get() {
                let new_x = event.offset_x() as f64;
                let new_y = event.offset_y() as f64;
                context_copy.line_to(new_x, new_y);
                context_copy.stroke();
            }
        }) as Box<dyn FnMut(_)>);

        canvas
            .add_event_listener_with_callback("mousemove", mouse_move.as_ref().unchecked_ref())?;

        mouse_move.forget();
    }

    Ok(())
}
