use super::map_sprite::MapSprite;
use super::ship_sprite::ShipSprite;
use super::text_sprite::{TextBox, TextSprite};
use super::trail_sprite::TrailSprite;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};

use super::ship::Ship;
use super::trail::Trail;

use super::transform::Transform2d;

use wasm_bindgen::prelude::wasm_bindgen;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn get_gl_context(canvas: &HtmlCanvasElement) -> Result<WebGl2RenderingContext, JsValue> {
    Ok(canvas.get_context("webgl2")?.unwrap().dyn_into()?)
}

pub struct Renderer {
    pub gl: WebGl2RenderingContext,
    canvas: HtmlCanvasElement,
    ship_sprite: ShipSprite,
    pub map_sprite: MapSprite,
    trail_sprite: TrailSprite,
    text_sprite: TextSprite,

    canvas_resolution: (u32, u32),
}

impl Renderer {
    pub fn new(canvas: HtmlCanvasElement) -> Result<Self, u8> {
        let gl = get_gl_context(&canvas).expect("No GL Canvas");

        gl.clear_color(0.0, 0.0, 0.0, 1.0);
        //gl.enable(WebGl2RenderingContext::DEPTH_TEST);
        gl.enable(WebGl2RenderingContext::BLEND);
        gl.clear(
            WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT,
        );

        if gl.is_null() {
            panic!("No Webgl");
        }

        let ship_sprite = match ShipSprite::new(&gl) {
            Ok(g) => g,
            Err(err) => {
                log(&format!("Ship Sprite error {:?}", err));
                panic!("Ship Sprite error");
            }
        };
        let map_sprite = match MapSprite::new(&gl) {
            Ok(g) => g,
            Err(err) => {
                log(&format!("Ship Sprite error {:?}", err));
                panic!("Ship Sprite error");
            }
        };

        let trail_sprite = match TrailSprite::new(&gl) {
            Ok(g) => g,
            Err(err) => {
                log(&format!("Ship Sprite error {:?}", err));
                panic!("Ship Sprite error");
            }
        };
        let text_sprite = match TextSprite::new(&gl) {
            Ok(g) => g,
            Err(err) => {
                log(&format!("Text Sprite error {:?}", err));
                panic!("Text Sprite error");
            }
        };

        Ok(Self {
            gl,
            canvas,
            ship_sprite,
            map_sprite,
            trail_sprite,
            text_sprite,
            canvas_resolution: (100, 100),
        })
    }

    pub fn render(
        &mut self,
        camera_transform: &Transform2d,
        ships: Vec<&Ship>,
        trails: Vec<&Trail>,
        text_boxes: Vec<&TextBox>,
    ) {
        // Rendering
        self.check_resize();
        self.gl.clear(
            WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT,
        );
        let screen_aspect_ratio =
            (self.canvas_resolution.1 as f32) / (self.canvas_resolution.0 as f32);
        let camera_to_clipspace = [1.0, 0.0, 0.0, 0.0, screen_aspect_ratio, 0.0, 0.0, 0.0, 1.0];

        let world_to_camera = camera_transform.to_mat3_array();
        self.trail_sprite.camera_to_clipspace = camera_to_clipspace;

        let world_to_trails = Transform2d::new(0.0, 0.0, 0.0, 1.0).to_mat3_array();

        self.trail_sprite.world_to_camera = world_to_camera;
        self.trail_sprite.world_to_sprite = world_to_trails;

        self.trail_sprite.setup(&self.gl);
        for trail in trails {
            self.trail_sprite.render(&self.gl, &trail);
        }

        self.ship_sprite.camera_to_clipspace = camera_to_clipspace;
        self.ship_sprite.world_to_camera = world_to_camera;
        self.ship_sprite.setup(&self.gl);
        for ship in ships {
            self.ship_sprite.render(&self.gl, &ship);
        }

        let map_sprite_transform = Transform2d::new(0.0, 0.0, 0.0, 1.0);
        // Render the map
        self.map_sprite.world_to_camera = world_to_camera;
        self.map_sprite.camera_to_clipspace = camera_to_clipspace;
        self.map_sprite.world_to_sprite = map_sprite_transform.to_mat3_array();
        self.map_sprite.render(&self.gl);

        self.text_sprite.setup(&self.gl);
        for text in text_boxes {
            self.text_sprite.render(&self.gl, text, screen_aspect_ratio);
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

            self.canvas_resolution = (client_width, client_height);
        }
    }
}
