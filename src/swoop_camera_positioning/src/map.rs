use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f32;
}

use super::transform::{length, normalize, PolarCoordinate, Vec2};
// TODO: rewrite map to be easily portable

pub struct Map {
    pub sin_consts: [f32; 8],
    pub cos_consts: [f32; 8],
    pub track_base_radius: f32,
    pub track_width: f32,
}

impl Map {
    pub fn track_radius(&self, angle: f32) -> f32 {
        let mut track_radius = self.track_base_radius;
        for i in 0..8 {
            let omega = (i + 1) as f32;
            track_radius += f32::sin(angle * omega) * self.sin_consts[i];
            track_radius += f32::cos(angle * omega) * self.cos_consts[i];
        }
        track_radius
    }

    pub fn distance_field(&self, position: Vec2) -> f32 {
        let course = length(position);
        let angle = position.0.atan2(position.1);

        let track_radius = self.track_radius(angle);

        let mut track_sdf = course - track_radius;
        track_sdf = f32::abs(track_sdf) - self.track_width;
        return track_sdf;
    }

    /// Uses finite difference to approximate the direction onto the
    /// track. This isn't quite the actual normal because the distance
    /// field isn't quite the distance field.
    pub fn calc_normal(&self, position: Vec2) -> Vec2 {
        const DELTA: f32 = 0.01;
        let here = self.distance_field(position);
        let above = self.distance_field((position.0, position.1 + DELTA));
        let right = self.distance_field((position.0 + DELTA, position.1));

        let dx = right - here;
        let dy = above - here;

        return normalize((dx, dy));
    }

    pub fn get_start_position(&self) -> PolarCoordinate {
        const ANGLE: f32 = 0.0;
        PolarCoordinate {
            angle: ANGLE,
            radius: self.track_radius(ANGLE),
        }
    }

    /// Returns the angle pointing along the track at a particular
    /// polar/angular coordinate along the track
    pub fn get_track_direction(&self, angle: f32) -> f32 {
        const DELTA_ANGLE: f32 = 0.01;
        let radius_here = self.track_radius(angle);
        let radius_a_bit_further = self.track_radius(angle + DELTA_ANGLE);
        let delta_radius = radius_here - radius_a_bit_further;

        // Use cosine rule to find the length of the line joining the
        // two radius' (chord)
        let joining_side_length = cosine_rule(radius_here, radius_a_bit_further, DELTA_ANGLE);

        // Use sin rule to find the angle of the chord and radius_here
        let ratio = radius_here / joining_side_length * f32::sin(DELTA_ANGLE);
        let ratio = f32::max(f32::min(ratio, 1.0), -1.0); // Floating point precision
        let extra_angle = f32::asin(ratio);

        if delta_radius.is_sign_negative() {
            -angle - extra_angle
        } else {
            -angle + extra_angle + std::f32::consts::PI
        }
    }

    /// Change the sin and cosine constants to change the map course
    pub fn randomize(&mut self) {
        const WAVINESS: f32 = 3.0;
        for i in 0..8 {
            let rand1 = (random() - 0.5) * 2.0;
            let rand2 = (random() - 0.5) * 2.0;
            let amplitude = WAVINESS / f32::powf((i + 1) as f32, 1.3);

            self.sin_consts[i] = rand1 * amplitude;
            self.cos_consts[i] = rand2 * amplitude;
        }
    }
}

pub fn cosine_rule(a: f32, b: f32, angle: f32) -> f32 {
    f32::sqrt(a * a + b * b - 2.0 * a * b * f32::cos(angle))
}
