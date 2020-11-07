use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{window, HtmlCanvasElement, KeyEvent, MouseEvent, WebGl2RenderingContext};

use super::shader_stl::ShaderStl;
use super::stl::Stl;
use super::textures::StaticTextures;

// Pull in the console.log function so we can debug things more easily
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub struct App {
    canvas: HtmlCanvasElement,
    gl: WebGl2RenderingContext,
    stl: Stl,
    shader_stl: ShaderStl,
}

impl App {
    pub fn new(canvas: HtmlCanvasElement) -> Self {
        let gl = get_gl_context(&canvas).expect("No GL Canvas");

        gl.clear_color(0.0, 0.0, 0.0, 1.0);
        gl.enable(WebGl2RenderingContext::DEPTH_TEST);
        gl.clear(
            WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT,
        );

        if gl.is_null() {
            panic!("No Webgl");
        }

        let stl = match Stl::new(&gl, include_bytes!("resources/monkey.stl")) {
            Ok(g) => g,
            Err(err) => {
                log(&format!("Stl error {:?}", err));
                panic!("Stl error");
            }
        };

        let mut shader_stl = match ShaderStl::new(&gl) {
            Ok(g) => g,
            Err(err) => {
                log(&format!("ShaderStl error {:?}", err));
                panic!("ShaderStl error");
            }
        };

        let textures = match StaticTextures::new(&gl) {
            Ok(g) => g,
            Err(err) => {
                log(&format!("StaticTexture error {:?}", err));
                panic!("StaticTexture error");
            }
        };

        shader_stl.image_matcap = Some(textures.stl_matcap.clone());

        Self {
            canvas,
            gl,
            stl,
            shader_stl,
        }
    }

    fn check_resize(&mut self) {
        let client_width = self.canvas.client_width();
        let client_height = self.canvas.client_height();
        let canvas_width = self.canvas.width() as i32;
        let canvas_height = self.canvas.height() as i32;

        if client_width != canvas_width || client_height != canvas_height {
            self.gl.viewport(0, 0, client_width, client_height);
            let client_width = client_width as u32;
            let client_height = client_height as u32;

            self.canvas.set_width(client_width);
            self.canvas.set_height(client_height);
            self.shader_stl.resolution = (client_width, client_height);

            log(&format!("Resized to {}:{}", client_width, client_height));
        }
    }

    pub fn animation_frame(&mut self) {
        self.check_resize();
        self.gl.clear(
            WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT,
        );

        let now = window().unwrap().performance().unwrap().now();
        let time = (now / 1000.0) as f32;
        self.shader_stl.time = time;

        self.shader_stl.setup(&self.gl);
        self.stl.render(&self.gl, &self.shader_stl);
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
