use wasm_bindgen::prelude::{wasm_bindgen};
use web_sys::{HtmlCanvasElement, MouseEvent, KeyEvent, WebGl2RenderingContext};
use wasm_bindgen::{JsCast, JsValue};

use super::triangle::{FirstTriangle};

// Pull in the console.log function so we can debug things more easily
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}


pub struct App {
    canvas: HtmlCanvasElement,
    gl: WebGl2RenderingContext,
    triangle: FirstTriangle,
}

impl App {
    pub fn new(canvas: HtmlCanvasElement) -> Self {
		let gl = get_gl_context(&canvas).expect("No GL Canvas");
		
		gl.clear_color(0.0, 0.0, 0.0, 1.0);
        gl.enable(WebGl2RenderingContext::DEPTH_TEST);
        gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT);

        if gl.is_null() {
            panic!("No Webgl");
        }
        let triangle = match FirstTriangle::new(&gl) {
            Ok(g) => g,
            Err(err) => {
                log(&format!("First Triangle error {:?}", err));
				panic!("First Triangle error");
            }
        };
        

        Self {
			canvas,
			gl,
			triangle
		}
    }

    pub fn animation_frame(&mut self) {
        self.gl.clear(
            WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT,
        );

        self.triangle.render(&self.gl);
    }

    pub fn mouse_event(&mut self, event: MouseEvent) {
        log(&format!("Mouse Event {:?}", event));
    }
    pub fn key_event(&mut self, event: KeyEvent) {
        log(&format!("Key Event {:?}", event));
    }

}


fn get_gl_context(canvas: &HtmlCanvasElement) -> Result<WebGl2RenderingContext, JsValue> {
    Ok(canvas.get_context("webgl2")?.unwrap().dyn_into()?)
}
