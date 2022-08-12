use glam::{Mat4, Vec3};

pub struct Camera {
    pub center: Vec3,
    pub elevation: f32,
    pub azimuth: f32,
    pub distance: f32,
    pub fov: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
}

use wasm_bindgen::prelude::wasm_bindgen;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

impl Camera {
    pub fn new() -> Self {
        Self {
            elevation: 0.0,
            azimuth: 0.0,
            distance: 15.0,
            fov: 1.0,
            near: 0.1,
            far: 500.0,
            aspect: 16.0 / 9.0,
            center: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    /// Converts to world_to_camera and camera_to_screen matrices
    pub fn to_matrices(&self) -> (Mat4, Mat4) {
        let sa = f32::sin(self.azimuth);
        let ca = f32::cos(self.azimuth);
        let se = f32::sin(self.elevation);
        let ce = f32::cos(self.elevation);
        let position = Vec3::new(
            self.distance * ca * ce,
            self.distance * sa * ce,
            self.distance * se,
        );
        let world_to_camera = Mat4::look_at_rh(
            self.center + position,
            self.center + Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        );

        let mut camera_to_screen =
            Mat4::perspective_rh_gl(self.fov, self.aspect, self.near, self.far);

        //~ let mut x_axis = camera_to_screen.x_axis();
        //~ x_axis[2] = x_axis[2] / self.far;
        //~ camera_to_screen.set_x_axis(x_axis);

        //~ let mut y_axis = camera_to_screen.y_axis();
        //~ y_axis[2] = y_axis[2] / self.far;
        //~ camera_to_screen.set_y_axis(y_axis);

        //~ let mut z_axis = camera_to_screen.z_axis();
        //~ z_axis[0] = z_axis[0] / self.far;
        //~ z_axis[1] = z_axis[1] / self.far;
        //        z_axis[2] = z_axis[2] / self.far;
        //~ camera_to_screen.set_z_axis(z_axis);

        (world_to_camera, camera_to_screen)
    }
}
