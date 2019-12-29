pub struct State {
    width: u32,
    height: u32,
    pen_thin: f64,
    color: String,
    preview_image: Vec<Vec<u8>>,
    undo_image_data: Vec<web_sys::ImageData>,
}

impl State {
    pub fn new(w: u32, h: u32) -> State {
        State {
            width: w,
            height: h,
            pen_thin: 1.0,                //TODO not hardcode
            color: "#000000".to_string(), //TODO not hardcode
            preview_image: vec![],
            undo_image_data: vec![],
        }
    }

    pub fn get_color(&self) -> String {
        self.color.clone() // not implement Copy trait
    }

    pub fn set_color(&mut self, color: String) {
        self.color = color;
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

    pub fn add_preview_image(&mut self, data: Vec<u8>) {
        self.preview_image.push(data);
    }

    //pub fn delete_image(&mut self) -> Result<()> {

    //}
}
