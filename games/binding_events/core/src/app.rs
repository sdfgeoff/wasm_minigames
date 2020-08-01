use wasm_bindgen::prelude::{wasm_bindgen};
use web_sys::{HtmlCanvasElement, MouseEvent, KeyEvent};

// Pull in the console.log function so we can debug things more easily
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}


pub struct App {
    canvas: HtmlCanvasElement,
}

impl App {
    pub fn new(canvas: HtmlCanvasElement) -> Self {
        Self { canvas }
    }

    pub fn animation_frame(&mut self) {
        log("Animation Frame")
    }

    pub fn mouse_event(&mut self, event: MouseEvent) {
        log(&format!("Mouse Event {:?}", event));
    }
    pub fn key_event(&mut self, event: KeyEvent) {
        log(&format!("Key Event {:?}", event));
    }
}
