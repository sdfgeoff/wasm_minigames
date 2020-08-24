use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{window, HtmlCanvasElement, KeyboardEvent, MouseEvent, WebGl2RenderingContext};

use super::map_sprite::MapSprite;
use super::ship::Ship;
use super::ship_sprite::ShipSprite;
use super::transform::Transform2d;

const CYAN_SHIP: (f32, f32, f32, f32) = (0.0, 0.5, 1.0, 1.0);
const YELLOW_SHIP: (f32, f32, f32, f32) = (1.0, 0.5, 0.0, 1.0);

// Pull in the console.log function so we can debug things more easily
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub struct App {
    canvas: HtmlCanvasElement,
    gl: WebGl2RenderingContext,
    ship_sprite: ShipSprite,
    map_sprite: MapSprite,
    key_map: KeyMap,

    prev_time: f64,

    ship_entities: Vec<Ship>,

    canvas_resolution: (u32, u32),
}

impl App {
    pub fn new(canvas: HtmlCanvasElement, options: String) -> Self {
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
        let map_sprite = match MapSprite::new(&gl, options) {
            Ok(g) => g,
            Err(err) => {
                log(&format!("Ship Sprite error {:?}", err));
                panic!("Ship Sprite error");
            }
        };

        let ship_entities = vec![
            Ship::new(CYAN_SHIP, Transform2d::new(0.0, 0.0, 0.0, 0.1)),
            Ship::new(YELLOW_SHIP, Transform2d::new(0.0, 0.1, 0.0, 0.1)),
        ];

        let now = window().unwrap().performance().unwrap().now();
        let prev_time = now / 1000.0;

        Self {
            canvas,
            gl,
            ship_sprite,
            map_sprite,
            key_map: KeyMap::new(),
            canvas_resolution: (0, 0),
            ship_entities,
            prev_time,
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

            log(&format!("Resized to {}:{}", client_width, client_height));
        }
    }

    pub fn animation_frame(&mut self) {
        // Logic
        let player_ship = &mut self.ship_entities[0];
        player_ship.linear_thrust = 0.0;
        player_ship.angular_thrust = 0.0;
        if self.key_map.forwards.active() {
            player_ship.linear_thrust += 1.0
        }
        if self.key_map.backwards.active() {
            player_ship.linear_thrust -= 1.0
        }
        if self.key_map.turn_left.active() {
            player_ship.angular_thrust += 1.0
        }
        if self.key_map.turn_right.active() {
            player_ship.angular_thrust -= 1.0
        }
        self.key_map.update();
        
        // Physics
        
        let now = window().unwrap().performance().unwrap().now();
        let time = now / 1000.0;

        let dt = time - self.prev_time;
        self.prev_time = time;

        for ship in &mut self.ship_entities {
            ship.update(dt as f32);
        }
        
        // Rendering

        self.check_resize();
        self.gl.clear(
            WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT,
        );

        let camera_transform = Transform2d::new(
            self.ship_entities[0].position.x,
            self.ship_entities[0].position.y,
            0.0,
            1.0 / self.canvas_resolution.1 as f32,
        );

        let world_to_camera = camera_transform.to_mat3_array();
        let camera_to_clipspace = [
            self.canvas_resolution.0 as f32,
            0.0,
            0.0,
            0.0,
            self.canvas_resolution.1 as f32,
            0.0,
            0.0,
            0.0,
            1.0,
        ];

        // Render all the ships
        self.ship_sprite.world_to_camera = world_to_camera;
        self.ship_sprite.camera_to_clipspace = camera_to_clipspace;

        for ship in &self.ship_entities {
            self.ship_sprite.world_to_sprite = ship.position.to_mat3_array();
            self.ship_sprite.ship_color = ship.color;
            self.ship_sprite.ship_engine = ship.linear_thrust;
            self.ship_sprite.render(&self.gl);
        }

        let map_sprite_transform = Transform2d::new(0.0, 0.0, 0.0, 1.0);
        // Render the map
        self.map_sprite.world_to_camera = world_to_camera;
        self.map_sprite.camera_to_clipspace = camera_to_clipspace;
        self.map_sprite.world_to_sprite = map_sprite_transform.to_mat3_array();
        self.map_sprite.render(&self.gl);
    }

    pub fn mouse_event(&mut self, _event: MouseEvent) {
        //log(&format!("Mouse Event {:?}", event));
    }
    pub fn keydown_event(&mut self, event: KeyboardEvent) {
        if !event.repeat() {
            self.key_map.set_state_from_str(&event.code(), KeyState::JustPressed);
        }
    }
    
    pub fn keyup_event(&mut self, event: KeyboardEvent) {
        self.key_map.set_state_from_str(&event.code(), KeyState::JustReleased);
    }
}

fn get_gl_context(canvas: &HtmlCanvasElement) -> Result<WebGl2RenderingContext, JsValue> {
    Ok(canvas.get_context("webgl2")?.unwrap().dyn_into()?)
}


#[derive(Debug)]
enum KeyState {
    JustPressed,
    Down,
    JustReleased,
    Up,
}

impl KeyState {
    fn update(&self) -> KeyState {
        match self {
            KeyState::JustPressed => KeyState::Down,
            KeyState::Down => KeyState::Down,
            KeyState::JustReleased => KeyState::Up,
            KeyState::Up => KeyState::Up,
        }
    }
    
    fn active(&self) -> bool {
        match self {
            KeyState::JustPressed => true,
            KeyState::Down => true,
            KeyState::JustReleased => false,
            KeyState::Up => false,
        }
    }
}


#[derive(Debug)]
struct KeyMap {
    forwards: KeyState,
    backwards: KeyState,
    turn_left: KeyState,
    turn_right: KeyState,
}

impl KeyMap {
    fn new() -> Self {
        Self {
            forwards: KeyState::Up,
            backwards: KeyState::Up,
            turn_left: KeyState::Up,
            turn_right: KeyState::Up,
        }
    }
    
    fn update(&mut self) {
        self.forwards = self.forwards.update();
        self.backwards = self.backwards.update();
        self.turn_left = self.turn_left.update();
        self.turn_right = self.turn_right.update();
    }
    
    
    fn set_state_from_str(&mut self, code: &str, new_state: KeyState) {
        match code {
            "KeyW" => {self.forwards = new_state},
            "KeyS" => {self.backwards = new_state},
            "KeyA" => {self.turn_left = new_state},
            "KeyD" => {self.turn_right = new_state},
            _ => ()
        };
    }
}
