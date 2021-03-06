use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{window, HtmlCanvasElement, KeyEvent, MouseEvent, WebGl2RenderingContext};

use super::background::Background;
use super::camera::Camera;
use super::shader_background::ShaderBackground;
use super::shader_stl::ShaderStl;
use super::stl::Stl;
use super::textures::StaticTextures;

use glam::{Mat4, Vec3};

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
    background: Background,
    shader_stl: ShaderStl,
    shader_background: ShaderBackground,
    camera: Camera,

    resolution: (u32, u32),
    click_location: Option<(i32, i32)>,

    dirty: bool,
    last_render_time: f32,
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

        let stl = match Stl::new(&gl, include_bytes!("resources/track.stl")) {
            Ok(g) => g,
            Err(err) => {
                log(&format!("Stl error {:?}", err));
                panic!("Stl error");
            }
        };
        let background = match Background::new(&gl) {
            Ok(g) => g,
            Err(err) => {
                log(&format!("Background error {:?}", err));
                panic!("Background error");
            }
        };

        let mut shader_stl = match ShaderStl::new(&gl) {
            Ok(g) => g,
            Err(err) => {
                log(&format!("ShaderStl error {:?}", err));
                panic!("ShaderStl error");
            }
        };
        let mut shader_background = match ShaderBackground::new(&gl) {
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
        shader_background.image_matcap = Some(textures.stl_matcap.clone());

        let camera = Camera::new();

        Self {
            canvas,
            gl,
            stl,
            background,
            shader_stl,
            shader_background,
            camera,
            resolution: (100, 100),
            click_location: None,
            dirty: true,
            last_render_time: 0.0,
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
            self.camera.aspect = (client_width as f32) / (client_height as f32);
            self.resolution = (client_width, client_height);

            self.dirty = true;

            log(&format!("Resized to {}:{}", client_width, client_height));
        }
    }

    pub fn animation_frame(&mut self) {
        self.check_resize();
        let now = window().unwrap().performance().unwrap().now();
        let time = (now / 1000.0) as f32;

        let time_since_render = time - self.last_render_time;
        if time_since_render > 0.2 {
            self.dirty = true;
        }

        if self.dirty {
            self.render();
            self.dirty = false;
            self.last_render_time = time;
        }
    }

    fn render(&mut self) {
        self.gl.clear(
            WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT,
        );

        let (world_to_camera, camera_to_screen) = self.camera.to_matrices();

        {
            // Render the background
            self.shader_background
                .setup(&self.gl, world_to_camera, camera_to_screen);
            self.background.render(&self.gl, &self.shader_background);
        }

        {
            // Render the models
            self.shader_stl
                .setup(&self.gl, world_to_camera, camera_to_screen);

            self.stl.world_to_model = Mat4::from_translation(Vec3::new(0.0, -25.0, -25.0));
            self.stl.color = Vec3::new(0.3, 0.3, 0.3);
            self.stl.render(&self.gl, &self.shader_stl);

            self.stl.color = Vec3::new(0.2, 0.2, 0.6);
            self.stl.world_to_model = Mat4::from_translation(Vec3::new(0.0, 25.0, -25.0));
            self.stl.render(&self.gl, &self.shader_stl);

            self.stl.world_to_model = Mat4::from_translation(Vec3::new(0.0, -25.0, 25.0));
            self.stl.color = Vec3::new(0.8, 0.8, 0.8);
            self.stl.render(&self.gl, &self.shader_stl);

            self.stl.world_to_model = Mat4::from_translation(Vec3::new(0.0, 25.0, 25.0));
            self.stl.color = Vec3::new(0.6, 0.2, 0.2);
            self.stl.render(&self.gl, &self.shader_stl);
        }
    }

    pub fn mouse_move(&mut self, event: MouseEvent) {
        const DRAG_SENSITIVITY: f32 = 5.0;
        match self.click_location {
            Some(location) => {
                let new = (event.client_x(), event.client_y());
                let delta = (location.0 - new.0, location.1 - new.1);
                self.click_location = Some(new);

                let percentage_x = (delta.0 as f32) / (self.resolution.0 as f32) * DRAG_SENSITIVITY;
                let percentage_y = (delta.1 as f32) / (self.resolution.0 as f32) * DRAG_SENSITIVITY;

                self.camera.azimuth += percentage_x;
                self.camera.elevation -= percentage_y;
                self.camera.elevation = f32::min(f32::max(self.camera.elevation, -1.4), 1.4);
                self.dirty = true;
            }
            None => {}
        }
    }
    pub fn mouse_down(&mut self, event: MouseEvent) {
        self.click_location = Some((event.client_x(), event.client_y()));
        self.dirty = true;
    }
    pub fn mouse_up(&mut self, _event: MouseEvent) {
        self.click_location = None;
        self.dirty = true;
    }

    pub fn key_event(&mut self, event: KeyEvent) {
        log(&format!("Key Event {:?}", event));
    }
}

fn get_gl_context(canvas: &HtmlCanvasElement) -> Result<WebGl2RenderingContext, JsValue> {
    Ok(canvas.get_context("webgl2")?.unwrap().dyn_into()?)
}
