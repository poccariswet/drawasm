use std::cell::Cell;
use std::f64;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent};

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

pub fn canvas_draw_start(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
    let context = canvas
        .get_context("2d")
        .expect("Could not get context")
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();
    let pressed = Rc::new(Cell::new(false));

    {
        let context_copy = context.clone();
        let canvas_copy = canvas.clone();
        let pressed = pressed.clone();
        let mouse_down = Closure::wrap(Box::new(move |event: MouseEvent| {
            let new_x = event.offset_x() as f64;
            let new_y = event.offset_y() as f64;
            context_copy.begin_path();
            context_copy.set_stroke_style(&JsValue::from("#000000"));
            context_copy.set_line_width(1.0);
            context_copy.move_to(new_x, new_y);
            pressed.set(true);
        }) as Box<dyn FnMut(_)>);

        canvas
            .add_event_listener_with_callback("mousedown", mouse_down.as_ref().unchecked_ref())?;

        mouse_down.forget();
    }

    {
        let pressed = pressed.clone();
        let mouse_up = Closure::wrap(Box::new(move |event: MouseEvent| {
            pressed.set(false);
        }) as Box<dyn FnMut(_)>);

        canvas.add_event_listener_with_callback("mouseup", mouse_up.as_ref().unchecked_ref())?;

        mouse_up.forget();
    }

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
