use std::cell::Cell;
use std::cell::RefCell;
use std::f64;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent};

use crate::state::State;

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
        let context = context.clone();
        let state = state.clone();
        let pressed = pressed.clone();

        let mouse_down = Closure::wrap(Box::new(move |event: MouseEvent| {
            pressed.set(true);
            let image_data = context
                .get_image_data(
                    0.0,
                    0.0,
                    state.borrow().get_width() as f64,
                    state.borrow().get_height() as f64,
                )
                .unwrap();
            state.borrow_mut().add_undo(image_data);

            let new_x = event.offset_x() as f64;
            let new_y = event.offset_y() as f64;
            context.begin_path();
            context.set_stroke_style(&JsValue::from(state.borrow().get_color()));
            context.set_line_width(state.borrow().get_pen_thin());
            context.move_to(new_x, new_y);
        }) as Box<dyn FnMut(_)>);

        canvas
            .add_event_listener_with_callback("mousedown", mouse_down.as_ref().unchecked_ref())?;

        mouse_down.forget(); // memory leak
    }

    // mouseup
    {
        let context = context.clone();
        let pressed = pressed.clone();

        let mouse_up = Closure::wrap(Box::new(move |event: MouseEvent| {
            pressed.set(false);
            let new_x = event.offset_x() as f64;
            let new_y = event.offset_y() as f64;
            context.fill_rect(new_x, new_y, 1.0, 1.0);
            context.line_to(new_x, new_y);
            context.stroke();
        }) as Box<dyn FnMut(_)>);

        canvas.add_event_listener_with_callback("mouseup", mouse_up.as_ref().unchecked_ref())?;

        mouse_up.forget();
    }

    // mousemove
    {
        let context = context.clone();
        let pressed = pressed.clone();

        let mouse_move = Closure::wrap(Box::new(move |event: MouseEvent| {
            if pressed.get() {
                let new_x = event.offset_x() as f64;
                let new_y = event.offset_y() as f64;
                context.line_to(new_x, new_y);
                context.stroke();
            }
        }) as Box<dyn FnMut(_)>);

        canvas
            .add_event_listener_with_callback("mousemove", mouse_move.as_ref().unchecked_ref())?;

        mouse_move.forget();
    }

    Ok(())
}
