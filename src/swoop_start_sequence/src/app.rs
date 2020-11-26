use wasm_bindgen::prelude::wasm_bindgen;

use web_sys::{window, HtmlCanvasElement, KeyboardEvent, MouseEvent};

use super::ai::calc_ai_control;
use super::camera::Camera;
use super::keymap::{KeyMap, KeyState};
use super::main_menu::MainMenu;
use super::map::Map;

use super::physics::calc_ship_physics;
use super::ship::Ship;

use super::renderer::Renderer;

use super::trail::Trail;
use super::transform::Transform2d;

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
    renderer: Renderer,
    map: Map,
    key_map: KeyMap,
    main_menu: MainMenu,

    prev_time: f64,

    ship_entities: Vec<Ship>,
    trails: Vec<Trail>,
    camera: Camera,

    game_state: GameState,
}

impl App {
    pub fn new(canvas: HtmlCanvasElement, _options: String) -> Self {
        let renderer = Renderer::new(canvas).expect("Failed to create renderer");

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

            trails.push(Trail::new(
                ship.color.clone(),
                MAIN_TRAIL_WIDTH,
                MAIN_TRAIL_BRIGHTNESS,
            ));
            trails.push(Trail::new(
                ship.color.clone(),
                WINGTIP_TRAIL_WIDTH,
                WINGTIP_TRAIL_BRIGHTNESS,
            ));
            trails.push(Trail::new(
                ship.color.clone(),
                WINGTIP_TRAIL_WIDTH,
                WINGTIP_TRAIL_BRIGHTNESS,
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
            renderer,
            main_menu: MainMenu::new(),
            map,
            camera,
            key_map: KeyMap::new(),
            ship_entities,
            trails,
            prev_time,
            game_state: GameState::Menu,
        };
        game.start_game();
        game
    }

    fn start_game(&mut self) {
        self.camera.reset();
        self.map.randomize();

        // TODO: this is a bit dodgy
        self.renderer
            .map_sprite
            .set_to_map(&self.renderer.gl, &self.map);

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
            for (ship_id, ship) in self.ship_entities.iter().enumerate() {
                self.trails[ship_id * 3].update(
                    dt as f32,
                    ship.get_engine_position(),
                    f32::abs(ship.linear_thrust),
                );

                let wingtip_positions = ship.get_wingtip_positions();

                let raw_slip = ship.calc_slip() / 2.5;
                let base_slip = f32::abs(raw_slip);
                let left_slip = base_slip + raw_slip / 8.0;
                let right_slip = base_slip - raw_slip / 8.0;

                self.trails[ship_id * 3 + 1].update(
                    dt as f32,
                    wingtip_positions.0,
                    f32::max(f32::min(left_slip, 1.0), 0.0),
                );
                self.trails[ship_id * 3 + 2].update(
                    dt as f32,
                    wingtip_positions.1,
                    f32::max(f32::min(right_slip, 1.0), 0.0),
                );
            }
        }

        let ship_entity_refs = self.ship_entities.iter().collect();
        let trail_entity_refs = self.trails.iter().collect();

        self.renderer.render(
            &self.camera.get_camera_matrix(),
            ship_entity_refs,
            trail_entity_refs,
            vec![],
        )
    }

    pub fn animation_frame(&mut self) {
        let now = window().unwrap().performance().unwrap().now();
        let time = now / 1000.0;
        let dt = time - self.prev_time;
        self.prev_time = time;

        match self.game_state {
            GameState::Menu => {
                self.show_logo(dt);
            }
            GameState::Playing => {
                self.play_game(dt);
            }
        }
    }

    pub fn show_logo(&mut self, dt: f64) {
        if self.key_map.start_game.active() {
            self.game_state = GameState::Playing;
            return;
        }
        self.main_menu.update(dt);

        let world_to_camera = Transform2d::new(0.0, -0.7, 0.0, 3.0);

        let ship_entity_refs = self.main_menu.logo.ships.iter().collect();
        let trail_entity_refs = self.main_menu.logo.trails.iter().collect();

        self.renderer.render(
            &world_to_camera,
            ship_entity_refs,
            trail_entity_refs,
            vec![&self.main_menu.text],
        );
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
