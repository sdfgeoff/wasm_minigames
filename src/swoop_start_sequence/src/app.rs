use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{window, HtmlCanvasElement, KeyboardEvent, MouseEvent, WebGl2RenderingContext};

use super::ai::calc_ai_control;
use super::camera::Camera;
use super::keymap::{KeyMap, KeyState};
use super::map::Map;
use super::map_sprite::MapSprite;
use super::physics::calc_ship_physics;
use super::ship::Ship;
use super::ship_sprite::ShipSprite;
use super::trail::Trail;
use super::trail_sprite::TrailSprite;
use super::transform::Transform2d;
use super::logo::Logo;
use super::text_sprite::TextSprite;

const YELLOW_SHIP: (f32, f32, f32, f32) = (1.0, 0.7, 0.0, 1.0);
const PINK_SHIP: (f32, f32, f32, f32) = (1.0, 0.0, 0.7, 1.0);
const PURPLE_SHIP: (f32, f32, f32, f32) = (0.7, 0.0, 1.0, 1.0);
const CYAN_SHIP: (f32, f32, f32, f32) = (0.0, 0.7, 1.0, 1.0);

//~ const WHITE_SHIP: (f32, f32, f32, f32) = (0.7, 0.7, 0.7, 1.0);
//~ const RED_SHIP: (f32, f32, f32, f32) = (1.0, 0.0, 0.0, 1.0);
//~ const GREEN_SHIP: (f32, f32, f32, f32) = (0.0, 1.0, 0.0, 1.0);
//~ const BLUE_SHIP: (f32, f32, f32, f32) = (0.0, 0.0, 1.0, 1.0);

// Pull in the console.log function so we can debug things more easily
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}


enum GameState {
    Menu,
    Playing,
}


pub struct App {
    canvas: HtmlCanvasElement,
    gl: WebGl2RenderingContext,
    ship_sprite: ShipSprite,
    map_sprite: MapSprite,
    trail_sprite: TrailSprite,
    map: Map,
    key_map: KeyMap,
    logo: Logo,

    prev_time: f64,

    ship_entities: Vec<Ship>,
    trails: Vec<(Trail, Trail, Trail)>,
    camera: Camera,
    
    text_sprite: TextSprite,

    canvas_resolution: (u32, u32),
    
    game_state: GameState,
}

impl App {
    pub fn new(canvas: HtmlCanvasElement, _options: String) -> Self {
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

        let ship_entities = vec![
            Ship::new(CYAN_SHIP),
            Ship::new(YELLOW_SHIP),
            Ship::new(PINK_SHIP),
            Ship::new(PURPLE_SHIP),
            //~ Ship::new(GREEN_SHIP),
            //~ Ship::new(BLUE_SHIP),
            //~ Ship::new(RED_SHIP),
            //~ Ship::new(WHITE_SHIP)
        ];

        let mut trails = vec![];
        for ship in ship_entities.iter() {
            const MAIN_TRAIL_WIDTH: f32 = 0.10;
            const WINGTIP_TRAIL_WIDTH: f32 = 0.02;
            const MAIN_TRAIL_BRIGHTNESS: f32 = 0.3;
            const WINGTIP_TRAIL_BRIGHTNESS: f32 = 1.0;

            trails.push((
                Trail::new(ship.color.clone(), MAIN_TRAIL_WIDTH, MAIN_TRAIL_BRIGHTNESS),
                Trail::new(
                    ship.color.clone(),
                    WINGTIP_TRAIL_WIDTH,
                    WINGTIP_TRAIL_BRIGHTNESS,
                ),
                Trail::new(
                    ship.color.clone(),
                    WINGTIP_TRAIL_WIDTH,
                    WINGTIP_TRAIL_BRIGHTNESS,
                ),
            ));
        }

        let map = Map {
            sin_consts: [2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            cos_consts: [0.0, -2.0, 0.0, 1.0, 0.0, 0.0, 0.5, 0.0],
            track_base_radius: 8.0,
            track_width: 0.7,
        };

        let camera = Camera::new();

        let now = window().unwrap().performance().unwrap().now();
        let prev_time = now / 1000.0;

        let mut game = Self {
            canvas,
            gl,
            ship_sprite,
            map_sprite,
            trail_sprite,
            logo: Logo::new(),
            map,
            camera,
            key_map: KeyMap::new(),
            canvas_resolution: (0, 0),
            ship_entities,
            trails,
            prev_time,
            text_sprite,
            game_state: GameState::Menu,
        };
        game.start_game();
        game
    }

    fn start_game(&mut self) {
        self.camera.reset();
        self.map.randomize();

        self.map_sprite.set_to_map(&self.gl, &self.map);

        {
            // Position the ships on the start line
            const SHIP_SPACING: f32 = 0.12;
            let start_position = self.map.get_start_position();
            let startline_angle = self.map.get_track_direction(start_position.angle);

            let startline_tangent = (f32::cos(startline_angle), f32::sin(startline_angle));
            let startline_normal = (-f32::sin(startline_angle), f32::cos(startline_angle));

            const NUM_START_COLUMNS: usize = 4;

            for (id, ship) in self.ship_entities.iter_mut().enumerate() {
                let row = id / NUM_START_COLUMNS;
                let column = id % NUM_START_COLUMNS;
                let column_offset = (column as f32) - ((NUM_START_COLUMNS - 1) as f32) * 0.5;
                let row_offset = row as f32;

                let offset_vec = (
                    (startline_tangent.0 * column_offset - startline_normal.0 * row_offset)
                        * SHIP_SPACING,
                    (startline_tangent.1 * column_offset - startline_normal.1 * row_offset)
                        * SHIP_SPACING,
                );

                let ship_start_position = start_position.to_cartesian();
                ship.position.x = ship_start_position.0 + offset_vec.0;
                ship.position.y = ship_start_position.1 + offset_vec.1;
                ship.position.rot = startline_angle;

                ship.velocity.x = 0.0;
                ship.velocity.y = 0.0;
                ship.velocity.rot = 0.0;
            }
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
    
    pub fn play_game(&mut self, dt: f64) {
        

        {
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

            let num_ships = self.ship_entities.len() - 2;
            for (id, ship) in self.ship_entities[1..].iter_mut().enumerate() {
                let skill = id as f32 / num_ships as f32;
                calc_ai_control(ship, skill, &self.map);
            }
        }
        {
            // Physics
            calc_ship_physics(&mut self.ship_entities, &self.map, dt as f32);
        }

        self.camera.target_position.0 = self.ship_entities[0].position.x;
        self.camera.target_position.1 = self.ship_entities[0].position.y;
        self.camera.target_velocity.0 = self.ship_entities[0].velocity.x;
        self.camera.target_velocity.1 = self.ship_entities[0].velocity.y;
        self.camera.update(dt as f32);

        {
            // Trails
            for (ship, trail) in self.ship_entities.iter().zip(self.trails.iter_mut()) {
                trail.0.update(
                    dt as f32,
                    ship.get_engine_position(),
                    f32::abs(ship.linear_thrust),
                );

                let wingtip_positions = ship.get_wingtip_positions();

                let raw_slip = ship.calc_slip() / 2.5;
                let base_slip = f32::abs(raw_slip);
                let left_slip = base_slip + raw_slip / 8.0;
                let right_slip = base_slip - raw_slip / 8.0;

                trail.1.update(
                    dt as f32,
                    wingtip_positions.0,
                    f32::max(f32::min(left_slip, 1.0), 0.0),
                );
                trail.2.update(
                    dt as f32,
                    wingtip_positions.1,
                    f32::max(f32::min(right_slip, 1.0), 0.0),
                );
            }
        }

        {
            // Rendering

            self.check_resize();
            self.gl.clear(
                WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT,
            );

            let world_to_camera = self.camera.get_camera_matrix();
            let camera_to_clipspace = [
                1.0,
                0.0,
                0.0,
                0.0,
                (self.canvas_resolution.1 as f32 / self.canvas_resolution.0 as f32),
                0.0,
                0.0,
                0.0,
                1.0,
            ];

            // Render all the ships
            self.ship_sprite.world_to_camera = world_to_camera;
            self.ship_sprite.camera_to_clipspace = camera_to_clipspace;
            self.ship_sprite.setup(&self.gl);
            for ship in &self.ship_entities {
                self.ship_sprite.render(&self.gl, ship);
            }

            let map_sprite_transform = Transform2d::new(0.0, 0.0, 0.0, 1.0);
            // Render the map
            self.map_sprite.world_to_camera = world_to_camera;
            self.map_sprite.camera_to_clipspace = camera_to_clipspace;
            self.map_sprite.world_to_sprite = map_sprite_transform.to_mat3_array();
            self.map_sprite.render(&self.gl);

            // Render the trails:
            self.trail_sprite.world_to_camera = world_to_camera;
            self.trail_sprite.camera_to_clipspace = camera_to_clipspace;
            self.trail_sprite.world_to_sprite = map_sprite_transform.to_mat3_array();
            self.trail_sprite.setup(&self.gl);
            for trail in &self.trails {
                self.trail_sprite.world_to_sprite = self.trail_sprite.world_to_camera;
                self.trail_sprite.render(&self.gl, &trail.0);
                self.trail_sprite.render(&self.gl, &trail.1);
                self.trail_sprite.render(&self.gl, &trail.2);
            }
        }
    }

    pub fn animation_frame(&mut self) {
        
        let now = window().unwrap().performance().unwrap().now();
        let time = now / 1000.0;
        let dt = time - self.prev_time;
        self.prev_time = time;
        
        match self.game_state {
            GameState::Menu => {
                self.show_logo(dt);
            },
            GameState::Playing => {
                self.play_game(dt);
            }
        }
    }
    
    pub fn show_logo(&mut self, _dt: f64) {
        if self.key_map.start_game.active() {
            self.game_state = GameState::Playing;
            return;
        }
        {
            // Rendering
            self.check_resize();
            self.gl.clear(
                WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT,
            );
            let camera_to_clipspace = [
                1.0,
                0.0,
                0.0,
                0.0,
                (self.canvas_resolution.1 as f32 / self.canvas_resolution.0 as f32),
                0.0,
                0.0,
                0.0,
                1.0,
            ];
            self.trail_sprite.camera_to_clipspace = camera_to_clipspace;
            
            let world_to_camera = Transform2d::new(0.0, -0.7, 0.0, 3.0).to_mat3_array();
            let world_to_trails = Transform2d::new(0.0, 0.0, 0.0, 1.0).to_mat3_array();
            
            self.trail_sprite.world_to_camera = world_to_camera;
            self.trail_sprite.world_to_sprite = world_to_trails;
            
            self.trail_sprite.setup(&self.gl);
            for trail in &self.logo.trails {
                self.trail_sprite.render(&self.gl, &trail);
            }
            
            self.ship_sprite.camera_to_clipspace = camera_to_clipspace;
            self.ship_sprite.world_to_camera = world_to_camera;
            for ship in &self.logo.ships {
                self.ship_sprite.setup(&self.gl);
                self.ship_sprite.render(&self.gl, &ship);
            }
            
            let map_sprite_transform = Transform2d::new(0.0, -1.0, 0.0, 0.1);
            // Render the map
            self.map_sprite.world_to_camera = world_to_camera;
            self.map_sprite.camera_to_clipspace = camera_to_clipspace;
            self.map_sprite.world_to_sprite = map_sprite_transform.to_mat3_array();
            self.map_sprite.render(&self.gl);
            
            
            self.text_sprite.setup(&self.gl);
            self.text_sprite.render(&self.gl);
        }
        
    }

    pub fn mouse_event(&mut self, _event: MouseEvent) {
        //log(&format!("Mouse Event {:?}", event));
    }
    pub fn keydown_event(&mut self, event: KeyboardEvent) {
        if !event.repeat() {
            self.key_map
                .set_state_from_str(&event.code(), KeyState::JustPressed);
        }
    }

    pub fn keyup_event(&mut self, event: KeyboardEvent) {
        self.key_map
            .set_state_from_str(&event.code(), KeyState::JustReleased);
    }
}

fn get_gl_context(canvas: &HtmlCanvasElement) -> Result<WebGl2RenderingContext, JsValue> {
    Ok(canvas.get_context("webgl2")?.unwrap().dyn_into()?)
}
