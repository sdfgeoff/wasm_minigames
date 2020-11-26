use super::map::Map;
use super::ship::Ship;
use super::trail::Trail;
use super::camera::Camera;
use super::keymap::KeyMap;
use super::text_sprite::TextBox;

use super::ai::calc_ai_control;
use super::physics::calc_ship_physics;


// Trail visuals
const MAIN_TRAIL_WIDTH: f32 = 0.10;
const WINGTIP_TRAIL_WIDTH: f32 = 0.02;
const MAIN_TRAIL_BRIGHTNESS: f32 = 0.3;
const WINGTIP_TRAIL_BRIGHTNESS: f32 = 1.0;

// Ship startline settings
const SHIP_SPACING: f32 = 0.12;
const NUM_START_COLUMNS: usize = 4;
            


const YELLOW_SHIP: (f32, f32, f32, f32) = (1.0, 0.7, 0.0, 1.0);
const PINK_SHIP: (f32, f32, f32, f32) = (1.0, 0.0, 0.7, 1.0);
const PURPLE_SHIP: (f32, f32, f32, f32) = (0.7, 0.0, 1.0, 1.0);
const CYAN_SHIP: (f32, f32, f32, f32) = (0.0, 0.7, 1.0, 1.0);
//~ const WHITE_SHIP: (f32, f32, f32, f32) = (0.7, 0.7, 0.7, 1.0);
//~ const RED_SHIP: (f32, f32, f32, f32) = (1.0, 0.0, 0.0, 1.0);
//~ const GREEN_SHIP: (f32, f32, f32, f32) = (0.0, 1.0, 0.0, 1.0);
//~ const BLUE_SHIP: (f32, f32, f32, f32) = (0.0, 0.0, 1.0, 1.0);


pub struct GamePlay {
    pub map: Map,
    pub ship_entities: Vec<Ship>,
    pub trails: Vec<Trail>,
    pub camera: Camera,

    pub countdown_text: TextBox,

    pub game_duration: f64,
}

impl GamePlay {
    pub fn new() -> Self {

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

        let countdown_text = TextBox::new((3,1), 0.2, (0.5, 0.5));

        Self {
            map,
            ship_entities,
            trails,
            camera,
            game_duration: -4.0,
            countdown_text,
        }
    }

    pub fn steer_ships(&mut self, key_map: &KeyMap) {
        // Player Ship
        let player_ship = &mut self.ship_entities[0];
        player_ship.linear_thrust = 0.0;
        player_ship.angular_thrust = 0.0;
        if key_map.forwards.active() {
            player_ship.linear_thrust += 1.0
        }
        if key_map.backwards.active() {
            player_ship.linear_thrust -= 1.0
        }
        if key_map.turn_left.active() {
            player_ship.angular_thrust += 1.0
        }
        if key_map.turn_right.active() {
            player_ship.angular_thrust -= 1.0
        }

        // Ai Ships
        let num_ships = self.ship_entities.len() - 2;
        for (id, ship) in self.ship_entities[1..].iter_mut().enumerate() {
            let skill = id as f32 / num_ships as f32;
            calc_ai_control(ship, skill, &self.map);
        }

    }


    pub fn update_trails(&mut self, dt: f64) {
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

    pub fn get_text_entities<'a>(&'a self) -> Vec<&'a TextBox> {
        if self.game_duration < 1.0 {
            vec![&self.countdown_text]
        } else {
            vec![]
        }
    }

    pub fn update(&mut self, dt: f64, key_map: &KeyMap) {
        self.game_duration += dt;
        if self.game_duration < 0.0 {
            // Do the countdown!
            self.countdown_text.clear();
            
            let remaining = -self.game_duration.floor();
            let diff = 1.0 - remaining - self.game_duration;

            self.countdown_text.append_string(
                &format!(" {} ", remaining as u8),
                &[0.0, diff as f32, 0.0]
            );
        } else {
            if self.game_duration < 1.0 {
                self.countdown_text.clear();
                self.countdown_text.append_string(&"Go!", &[0.0, 1.0 - self.game_duration as f32, 0.0]);
            }
            calc_ship_physics(&mut self.ship_entities, &self.map, dt as f32);
        }

        self.steer_ships(key_map);
        self.update_trails(dt);

        self.camera.target_position.0 = self.ship_entities[0].position.x;
        self.camera.target_position.1 = self.ship_entities[0].position.y;
        self.camera.target_velocity.0 = self.ship_entities[0].velocity.x;
        self.camera.target_velocity.1 = self.ship_entities[0].velocity.y;
        self.camera.update(dt as f32);
    }

    pub fn reset(&mut self) {
        self.camera.reset();
        self.map.randomize();

        {
            // Position the ships on the start line
            
            let start_position = self.map.get_start_position();
            let startline_angle = self.map.get_track_direction(start_position.angle);

            let startline_tangent = (f32::cos(startline_angle), f32::sin(startline_angle));
            let startline_normal = (-f32::sin(startline_angle), f32::cos(startline_angle));

            

            for (id, ship) in self.ship_entities.iter_mut().enumerate() {
                let row = id / NUM_START_COLUMNS;
                let column = id % NUM_START_COLUMNS;
                let column_offset = (column as f32) - ((NUM_START_COLUMNS - 1) as f32) * 0.5;
                let row_offset = row as f32 + 0.5;

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
}