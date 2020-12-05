use wasm_bindgen::prelude::wasm_bindgen;

// Pull in the console.log function so we can debug things more easily
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// This struct will be accessible from JS as a JS object that can be
// created using `new Core()`
#[wasm_bindgen]
pub struct Core {}

#[wasm_bindgen]
impl Core {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: String) -> Self {
        log(&format!("WASM Started for canvas {}", canvas_id));
        Self {}
    }

    #[wasm_bindgen]
    pub fn start(&mut self) {
        log("App Started");
    }
}
