use super::transform::Vec2;
use std::collections::VecDeque;

const TIME_PER_SEGMENT: f32 = 0.25;
const NUM_SEGMENTS: usize = 20; // Should be half the "point_buffer" length in the vertex shader as it takes two vec4's to represent the data

pub struct PathPoint {
    pub position: Vec2,
    pub tangent: Vec2,
    pub intensity: f32,
    pub width: f32,
    pub brightness: f32,
}

pub struct Trail {
    pub path: VecDeque<PathPoint>,
    pub color: (f32, f32, f32, f32),
    pub width: f32,
    pub brightness: f32,
    max_length: usize,
    time_since_emit: f32,
    prev_position: Vec2,
}

impl Trail {
    pub fn new(color: (f32, f32, f32, f32), width: f32, brightness: f32) -> Self {
        Self {
            path: VecDeque::new(),
            color,
            max_length: NUM_SEGMENTS,
            prev_position: (0.0, 0.0),
            time_since_emit: 0.0,
            width,
            brightness,
        }
    }

    pub fn update(&mut self, dt: f32, position: Vec2, intensity: f32) {
        self.time_since_emit += dt;

        // Ensure the path is completely full of points. Because they
        // have a tangent of zero, they will render with zero width
        // so not be visible.
        if self.path.len() != self.max_length {
            self.path.clear();
            for _ in 0..self.max_length {
                self.path.push_back(PathPoint {
                    position: position,
                    tangent: (0.0, 0.0),
                    intensity: 0.0,
                    brightness: 0.0,
                    width: 0.0,
                });
            }
            assert!(self.path.len() == self.max_length)
        }

        // Find the ships actual velocity at this instant of time
        let current_tangent = (
            (self.prev_position.0 - position.0) / dt,
            (self.prev_position.1 - position.1) / dt,
        );
        self.prev_position.0 = position.0;
        self.prev_position.1 = position.1;

        // If it's time to add a new segment, rotate the array, making
        // the current zeroth PathPoint into the first PathPoint, the
        // first PathPoint into the second PathPoint etc.
        if self.time_since_emit > TIME_PER_SEGMENT {
            self.path.rotate_right(1);
            self.time_since_emit = dt; // If this is zero, the tangent = 0
        }

        {
            // Update the zeroth PathPoint with information about the
            // ship from this instant.
            let first = self.path.get_mut(0).expect("path invalid");
            first.position.0 = position.0;
            first.position.1 = position.1;
            first.tangent.0 = current_tangent.0 * self.time_since_emit;
            first.tangent.1 = current_tangent.1 * self.time_since_emit;
            first.intensity = intensity;
            first.brightness = self.brightness;
            first.width = self.width;
        }
    }

    pub fn length(&self) -> i32 {
        self.path.len() as i32
    }

    /// Because the trail is divided into segments, the segments
    /// position (segment_id / chain_length) does not precisely
    /// represent it's distance from the head of the chain. This number
    /// represents the difference between a segments position in the
    /// chain and it's actual distance from the head.
    /// To get the trail to fade smoothly, you can use the formula:
    /// `distance_from_head = interpolated_segment_id / chain_length + offset`
    pub fn get_percent_offset(&self) -> f32 {
        (1.0 - self.time_since_emit / TIME_PER_SEGMENT) / ((self.max_length - 2) as f32)
    }

    /// Converts to a buffer containing position/tangent data and
    /// containing intensity data.
    /// Each "point" is 8 float values:
    /// position (x and y)
    /// tangent (x and y)
    /// brightness,
    /// width,
    /// placeholder
    pub fn path_data_buffers(&self) -> Vec<f32> {
        let mut point_buffer = vec![];

        for point in self.path.iter() {
            point_buffer.push(point.position.0);
            point_buffer.push(point.position.1);
            point_buffer.push(point.tangent.0);
            point_buffer.push(point.tangent.1);

            point_buffer.push(point.intensity);
            point_buffer.push(point.brightness);
            point_buffer.push(point.width);
            point_buffer.push(0.0);
        }

        point_buffer
    }
}
