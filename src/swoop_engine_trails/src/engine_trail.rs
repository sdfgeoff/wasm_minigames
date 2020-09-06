use super::transform::Vec2;
use std::collections::VecDeque;

const TIME_PER_SEGMENT: f32 = 0.25;
const NUM_SEGMENTS: usize = 20;


pub struct PathPoint {
    pub position: Vec2,
    pub tangent: Vec2,
    pub intensity: f32,
}

pub struct EngineTrail {
    path: VecDeque<PathPoint>,
    pub color: (f32, f32, f32, f32),
    max_length: usize,
    time_since_emit: f32,
    prev_position: Vec2,
}

impl EngineTrail {
    pub fn new(color: (f32, f32, f32, f32)) -> Self {
        Self {
            path: VecDeque::new(),
            color,
            max_length: NUM_SEGMENTS,
            prev_position: (0.0, 0.0),
            time_since_emit: 0.0,
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
                    intensity: intensity,
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

    /// Converts to a buffer containing position/tangent data and one
    /// containing intensity data
    pub fn path_data_buffers(&self) -> (Vec<f32>, Vec<f32>) {
        let mut point_buffer = vec![];
        let mut data_buffer = vec![];

        for point in self.path.iter() {
            point_buffer.push(point.position.0);
            point_buffer.push(point.position.1);
            point_buffer.push(point.tangent.0);
            point_buffer.push(point.tangent.1);
            
            data_buffer.push(point.intensity);
            data_buffer.push(0.0);
            data_buffer.push(0.0);
            data_buffer.push(0.0);
        }

        (point_buffer, data_buffer)
    }
}
