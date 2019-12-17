pub struct State {
    pen_thin: f64,
    color: String,
}

impl State {
    pub fn new() -> State {
        State {
            pen_thin: 1.0,                //TODO not hardcode
            color: "#000000".to_string(), //TODO not hardcode
        }
    }
}
