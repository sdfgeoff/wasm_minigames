use js_sys::Math::random;

use super::transform::{length, normalize, PolarCoordinate, Vec2};

pub struct Map {
    pub sin_consts: [f32; 8],
    pub cos_consts: [f32; 8],
    pub track_base_radius: f32,
    pub track_width: f32,
}

impl Map {
    /// Compute what radius the track has at a given angle from the track
    /// center.
    pub fn track_radius(&self, angle: f32) -> f32 {
        let mut track_radius = self.track_base_radius;
        for i in 0..8 {
            let omega = (i + 1) as f32;
            track_radius += f32::sin(angle * omega) * self.sin_consts[i];
            track_radius += f32::cos(angle * omega) * self.cos_consts[i];
        }
        track_radius
    }

    /// Computes the distance from the edge of the track for a given
    /// Cartesian coordinate. This can be used to check if a coordinate
    /// is inside or outside the track, and is negative inside the track
    /// and positive outside the track.
    pub fn distance_field(&self, position: Vec2) -> f32 {
        let course = length(&position);
        let angle = position.1.atan2(position.0);

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

    /// Figure out where the start line should be located. This is
    /// represented as a polar coordinate from the center of the track.
    pub fn get_start_position(&self) -> PolarCoordinate {
        const ANGLE: f32 = std::f32::consts::PI / 2.0;
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
        let delta_radius = radius_a_bit_further - radius_here;

        // Use cosine rule to find the length of the line joining the
        // two radius' (chord)
        let joining_side_length = cosine_rule(radius_here, radius_a_bit_further, DELTA_ANGLE);

        // Use sin rule to find the angle of the chord and radius_here
        let ratio = radius_here / joining_side_length * f32::sin(DELTA_ANGLE);
        let ratio = f32::max(f32::min(ratio, 1.0), -1.0); // Floating point precision
        let extra_angle = f32::asin(ratio);

        if delta_radius.is_sign_negative() {
            angle - extra_angle - std::f32::consts::PI / 2.0
        } else {
            angle + extra_angle + std::f32::consts::PI / 2.0
        }
    }

    /// Change the sin and cosine constants to change the map course
    pub fn randomize(&mut self) {
        const WAVINESS: f32 = 3.0;
        for i in 0..8 {
            let rand1 = (random() as f32 - 0.5) * 2.0;
            let rand2 = (random() as f32 - 0.5) * 2.0;
            let amplitude = WAVINESS / f32::powf((i + 1) as f32, 1.3);

            self.sin_consts[i] = rand1 * amplitude;
            self.cos_consts[i] = rand2 * amplitude;
        }
    }
    
    pub fn calc_position_on_track(&self, position: Vec2) -> f32 {
        let ship_angle = PolarCoordinate::from_cartesian(position).angle;
        let start_angle = self.get_start_position().angle;
        
        // Convert from angle to the range (0.0, 1.0)
        let ship_progress = (ship_angle + std::f32::consts::PI) / (2.0 * std::f32::consts::PI);
        let start_progress = (start_angle + std::f32::consts::PI) / (2.0 * std::f32::consts::PI);
        
        let mut abs_progress = start_progress - ship_progress;
        if abs_progress < 0.0 {
            abs_progress += 1.0;
        }
        
        abs_progress
    }
}

/// Cosine rule/law of cosines:
/// https://en.wikipedia.org/wiki/Law_of_cosines
pub fn cosine_rule(a: f32, b: f32, angle: f32) -> f32 {
    f32::sqrt(a * a + b * b - 2.0 * a * b * f32::cos(angle))
}
