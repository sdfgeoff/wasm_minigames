use super::map::Map;
use super::ship::Ship;
use super::transform::{PolarCoordinate, Vec2};
use std::f32::consts::PI;

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub fn calc_ai_control(ship: &mut Ship, skill: f32, map: &Map) {
    let mut steering = 0.0;
    let mut thrust = 0.0;

    let lookahead_mul = skill;

    steering += calc_steering_input(&ship, &map, 1.0 * lookahead_mul) * 0.15;
    steering += calc_steering_input(&ship, &map, 0.5 * lookahead_mul) * 0.45;
    steering += calc_steering_input(&ship, &map, 0.2 * lookahead_mul) * 0.4;

    thrust += 1.0;

    ship.angular_thrust = f32::max(f32::min(steering, 1.0), -1.0);
    ship.linear_thrust = f32::max(f32::min(thrust, 1.0), -1.0);
}

fn calc_steering_input(ship: &Ship, map: &Map, lookahead_time: f32) -> f32 {
    let polar_position = PolarCoordinate::from_cartesian(predict_position(ship, lookahead_time));

    let track_angle_here = map.get_track_direction(polar_position.angle);
    let track_radius_here = map.track_radius(polar_position.angle);

    let mut steering = 0.0;

    let radius_error = track_radius_here - polar_position.radius;
    let radius_steering_input = f32::max(f32::min(radius_error, PI / 2.0), -PI / 2.0);

    let mut target_angle = 0.0;
    target_angle += track_angle_here; // Face direction of track
    target_angle += radius_steering_input; // Fly towards track center

    let angular_error = wrap_angle(target_angle - ship.position.rot);
    steering += angular_error;

    steering
}

fn predict_position(ship: &Ship, time: f32) -> Vec2 {
    (
        ship.position.x + ship.velocity.x * time,
        ship.position.y + ship.velocity.y * time,
    )
}

/// Ensure a number is between pi and -pi
/// Not sure if this is the optimal way, but it works
fn wrap_angle(angle: f32) -> f32 {
    let angle = angle + PI; // Work between 0 and 2PI;
    let sig = f32::signum(angle);
    let mag = f32::abs(angle) % (2.0 * PI);

    return sig * (mag - PI);
}
