use super::transform::{Transform2d, Vec2};

const ENGINE_THRUST: f32 = 10.0;
const TURNING_THRUST: f32 = 40.0;
const LINEAR_DAMPING: f32 = 2.0;
const ANGULAR_DAMPING: f32 = 8.0;

#[derive(Debug)]
pub struct Ship {
    pub position: Transform2d,
    pub velocity: Transform2d,
    pub linear_thrust: f32,
    pub angular_thrust: f32,
    pub color: (f32, f32, f32, f32),
}

impl Ship {
    pub fn new(color: (f32, f32, f32, f32), start_transform: Transform2d) -> Self {
        Ship {
            position: start_transform,
            velocity: Transform2d::new(0.0, 0.0, 0.0, 0.0),
            linear_thrust: 0.0,
            angular_thrust: 0.0,
            color: color,
        }
    }

    pub fn update(&mut self, dt: f32) {
        let angle: f32 = self.position.rot;

        let c = f32::cos(angle);
        let s = f32::sin(angle);

        let forwards = (-s, c);

        let mut acceleration = (0.0, 0.0, 0.0);
        acceleration.0 += forwards.0 * self.linear_thrust * ENGINE_THRUST;
        acceleration.1 += forwards.1 * self.linear_thrust * ENGINE_THRUST;
        acceleration.2 += self.angular_thrust * TURNING_THRUST;

        acceleration.0 -= self.velocity.x * LINEAR_DAMPING;
        acceleration.1 -= self.velocity.y * LINEAR_DAMPING;
        acceleration.2 -= self.velocity.rot * ANGULAR_DAMPING;

        self.velocity.x += acceleration.0 * dt;
        self.velocity.y += acceleration.1 * dt;
        self.velocity.rot += acceleration.2 * dt;

        // Integration
        self.position.x += self.velocity.x * dt;
        self.position.y += self.velocity.y * dt;
        self.position.rot += self.velocity.rot * dt;

        self.position.rot = wrap_angle(self.position.rot);
    }

    pub fn get_engine_position(&self) -> Vec2 {
        let offset = self.position.transform_vec((0.0, -0.4));
        (self.position.x + offset.0, self.position.y + offset.1)
    }
}

/// Ensure a number is between pi and -pi
/// Not sure if this is the optimal way, but it works
fn wrap_angle(angle: f32) -> f32 {
    let angle = angle + std::f32::consts::PI; // Work between 0 and 2PI;
    let sig = f32::signum(angle);
    let mag = f32::abs(angle) % (2.0 * std::f32::consts::PI);

    return sig * (mag - std::f32::consts::PI);
}
