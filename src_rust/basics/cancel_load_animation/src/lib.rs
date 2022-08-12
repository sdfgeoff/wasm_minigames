use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlCanvasElement};

// Pull in the console.log function so we can debug things more easily
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// This struct will be accessible from JS as a JS object that can be
// created using `new Core()`
#[wasm_bindgen]
pub struct Core {
    _canvas: HtmlCanvasElement,
}

#[wasm_bindgen]
impl Core {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement) -> Self {
        log(&format!("WASM Started for canvas {}", canvas.id()));

        canvas.set_class_name("loaded");

        Self { _canvas: canvas }
    }

    #[wasm_bindgen]
    pub fn start(&mut self) {
        log("App Started");
    }
}
