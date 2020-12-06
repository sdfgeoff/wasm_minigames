use super::camera::Camera;
use super::keymap::KeyMap;
use super::map::Map;
use super::score::Score;
use super::ship::Ship;
use super::text_sprite::TextBox;
use super::trail::Trail;

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

const NUM_LAPS_TO_WIN: usize = 6;

const COUNTDOWN_TIME: f64 = 4.0;

const YELLOW_SHIP: (f32, f32, f32, f32) = (1.0, 0.7, 0.0, 1.0);
const PINK_SHIP: (f32, f32, f32, f32) = (1.0, 0.0, 0.7, 1.0);
const PURPLE_SHIP: (f32, f32, f32, f32) = (0.7, 0.0, 1.0, 1.0);
const CYAN_SHIP: (f32, f32, f32, f32) = (0.0, 0.7, 1.0, 1.0);
// const WHITE_SHIP: (f32, f32, f32, f32) = (0.7, 0.7, 0.7, 1.0);
// const RED_SHIP: (f32, f32, f32, f32) = (1.0, 0.0, 0.0, 1.0);
// const GREEN_SHIP: (f32, f32, f32, f32) = (0.0, 1.0, 0.0, 1.0);
// const BLUE_SHIP: (f32, f32, f32, f32) = (0.0, 0.0, 1.0, 1.0);

pub struct GamePlay {
    pub map: Map,
    pub ship_entities: Vec<Ship>,
    pub scores: Vec<Score>,
    pub trails: Vec<Trail>,
    pub camera: Camera,

    pub countdown_text: TextBox,
    pub leaderboard_text: TextBox,

    pub game_duration: f64,
}

impl GamePlay {
    pub fn new() -> Self {
        let ship_entities = vec![
            Ship::new(CYAN_SHIP),
            Ship::new(YELLOW_SHIP),
            Ship::new(PINK_SHIP),
            Ship::new(PURPLE_SHIP),
            // Ship::new(GREEN_SHIP),
            // Ship::new(BLUE_SHIP),
            // Ship::new(RED_SHIP),
            // Ship::new(WHITE_SHIP)
        ];

        let mut trails = vec![];
        let mut scores = vec![];

        for ship in ship_entities.iter() {
            scores.push(Score::new());

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

        let countdown_text = TextBox::new((3, 1), 0.2, (0.5, 0.5));
        let leaderboard_text =
            TextBox::new((7, (ship_entities.len() + 1) as i32), 0.05, (1.0, 0.5));

        Self {
            map,
            ship_entities,
            trails,
            scores,
            camera,
            game_duration: -COUNTDOWN_TIME,
            countdown_text,
            leaderboard_text,
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
            let mut skill = id as f32 / num_ships as f32;
            skill = skill * 0.5 + 0.2;
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
            vec![&self.leaderboard_text]
        }
    }

    pub fn update(&mut self, dt: f64, key_map: &KeyMap) {
        self.game_duration += dt;
        if self.game_duration < 0.0 {
            // Do the countdown!
            self.countdown_text.clear();

            let remaining = -self.game_duration.floor();
            let diff = 1.0 - remaining - self.game_duration;

            self.countdown_text
                .append_string(&format!(" {} ", remaining as u8), &[0.0, diff as f32, 0.0]);
        } else {
            if self.game_duration < 1.0 {
                self.countdown_text.clear();
                self.countdown_text
                    .append_string(&"Go!", &[0.0, 1.0 - self.game_duration as f32, 0.0]);
            } else {
                self.generate_leaderboard_text();
            }
            calc_ship_physics(&mut self.ship_entities, &self.map, dt as f32);

            for (ship, score) in self.ship_entities.iter().zip(self.scores.iter_mut()) {
                score.update(&self.map, ship, self.game_duration);
            }
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
        self.game_duration = -COUNTDOWN_TIME;

        {
            // Position the ships on the start line

            let start_position = self.map.get_start_position();
            let startline_angle = self.map.get_track_direction(start_position.angle);

            let startline_tangent = (f32::cos(startline_angle), f32::sin(startline_angle));
            let startline_normal = (-f32::sin(startline_angle), f32::cos(startline_angle));

            let ship_start_position = start_position.to_cartesian();

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

                ship.position.x = ship_start_position.0 + offset_vec.0;
                ship.position.y = ship_start_position.1 + offset_vec.1;
                ship.position.rot = startline_angle;

                ship.velocity.x = 0.0;
                ship.velocity.y = 0.0;
                ship.velocity.rot = 0.0;

                self.scores[id].reset(&self.map, ship);
            }
        }
        for trail in self.trails.iter_mut() {
            trail.reset();
        }
    }
    pub fn generate_leaderboard_text(&mut self) {
        self.leaderboard_text.clear();

        let mut ship_and_score_refs: Vec<(&Ship, &Score)> =
            self.ship_entities.iter().zip(self.scores.iter()).collect();
        ship_and_score_refs.sort_by(|a, b| a.1.cmp(b.1));
        let winner_score = ship_and_score_refs.first().expect("No Ships").1;

        self.leaderboard_text.append_string(
            &format!(
                "Lap {}/{}",
                winner_score.laps.len() - 1,
                NUM_LAPS_TO_WIN - 1
            ),
            &[0.5, 0.5, 0.5],
        );
        for (ship, score) in ship_and_score_refs {
            let color = [ship.color.0, ship.color.1, ship.color.2];
            if score.laps.len() == winner_score.laps.len() {
                if let Some(winner_time) = winner_score.laps.last() {
                    // Same lap - display time
                    let time = score.laps.last().unwrap() - winner_time;
                    let seconds = time as u32;
                    let millis = (time.fract() * 100.0).floor() as u32;
                    self.leaderboard_text
                        .append_string(&format!("~ {:02}:{:02}", seconds, millis), &color);
                } else {
                    // No-one has any time yet
                    self.leaderboard_text
                        .append_string(&format!("~ --:--",), &color);
                }
            } else {
                // This player is at least a lap behind
                self.leaderboard_text
                    .append_string(&format!("~ --:--",), &color);
            }
        }
    }

    /// Returns True when the game is complete.
    /// The game is considered complete when everyone has
    /// done enough laps
    pub fn game_complete(&self) -> bool {
        for score in self.scores.iter() {
            if score.laps.len() < NUM_LAPS_TO_WIN {
                return false;
            }
        }
        return true;
    }
}
