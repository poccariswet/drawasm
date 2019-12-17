pub struct State {
    drawing_ok: bool,
    pen_thin: f64,
    color: String,
}

impl State {
    pub fn new() {
        State {
            drawing_ok: false,
            pen_thin: 1.0,    //TODO not hardcode
            color: "#000000", //TODO not hardcode
        }
    }
}
