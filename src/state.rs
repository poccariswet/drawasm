pub struct State {
    width: u32,
    height: u32,
    pen_thin: f64,
    color: String,
    undo_image_data: Vec<web_sys::ImageData>,
}

impl State {
    pub fn new(w: u32, h: u32) -> State {
        State {
            width: w,
            height: h,
            pen_thin: 1.0,                //TODO not hardcode
            color: "#000000".to_string(), //TODO not hardcode
            undo_image_data: vec![],
        }
    }

    pub fn get_color(&self) -> String {
        self.color.clone() // not implement Copy trait
    }

    pub fn get_pen_thin(&self) -> f64 {
        self.pen_thin
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn add_undo(&mut self, data: web_sys::ImageData) {
        self.undo_image_data.push(data);
    }

    pub fn get_undo(&mut self) -> Option<web_sys::ImageData> {
        self.undo_image_data.pop()
    }
}
