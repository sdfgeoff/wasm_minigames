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
    pub fn new(canvas_id: String) -> Self {
        log(&format!("WASM Started for canvas {}", canvas_id));

        let selector = format!("#{}", canvas_id);

        let window = window().unwrap();
        let document = window.document().unwrap();
        let element = document
            .query_selector(&selector)
            .expect("Call failed")
            .expect("No element with selector");

        element.set_class_name("loaded");

        let canvas: HtmlCanvasElement = element.dyn_into().expect("Not a canvas");

        Self { _canvas: canvas }
    }

    #[wasm_bindgen]
    pub fn start(&mut self) {
        log("App Started");
    }
}
