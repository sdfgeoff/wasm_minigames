use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{window, HtmlCanvasElement, KeyEvent, MouseEvent, WebGl2RenderingContext};

use super::camera::Camera;
use super::framebuffer::GBuffer;
use super::resources::Resources;
use super::shader_background::ShaderBackground;
use super::shader_lighting_pass::ShaderLightingPass;
use super::shader_stl::ShaderStl;

use glam::{Mat4, Vec3, Vec4};

// Pull in the console.log function so we can debug things more easily
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub struct App {
    canvas: HtmlCanvasElement,
    gl: WebGl2RenderingContext,
    shader_stl: ShaderStl,
    shader_background: ShaderBackground,
    shader_lighting_pass: ShaderLightingPass,
    camera: Camera,
    resources: Resources,

    gbuffer: GBuffer,

    resolution: (u32, u32),
    click_location: Option<(i32, i32)>,
}

impl App {
    pub fn new(canvas: HtmlCanvasElement) -> Self {
        let gl = get_gl_context(&canvas).expect("No GL Canvas");

        let float_tex_extension = gl.get_extension("EXT_color_buffer_float");

        gl.clear_color(0.0, 0.0, 0.0, 1.0);
        gl.enable(WebGl2RenderingContext::DEPTH_TEST);
        gl.clear(
            WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT,
        );

        if gl.is_null() {
            panic!("No Webgl");
        }

        let resolution = (100, 100);

        let resources = match Resources::new(&gl) {
            Ok(g) => g,
            Err(err) => {
                log(&format!("Resource error {:?}", err));
                panic!("Resource error");
            }
        };

        let shader_stl = match ShaderStl::new(&gl, &resources) {
            Ok(g) => g,
            Err(err) => {
                log(&format!("ShaderStl error {:?}", err));
                panic!("ShaderStl error");
            }
        };
        let mut shader_background = match ShaderBackground::new(&gl, &resources) {
            Ok(g) => g,
            Err(err) => {
                log(&format!("ShaderStl error {:?}", err));
                panic!("ShaderStl error");
            }
        };
        let shader_lighting_pass = match ShaderLightingPass::new(&gl, &resources) {
            Ok(g) => g,
            Err(err) => {
                log(&format!("ShaderStl error {:?}", err));
                panic!("ShaderStl error");
            }
        };

        shader_background.image_matcap = resources.png_images.matcap.clone();

        let gbuffer = GBuffer::new(&gl, resolution).expect("Failed to create GBuffer");

        let camera = Camera::new();

        Self {
            canvas,
            gl,
            resources,
            shader_stl,
            shader_background,
            shader_lighting_pass,
            gbuffer,
            camera,
            resolution,
            click_location: None,
        }
    }

    fn check_resize(&mut self) {
        let client_width = self.canvas.client_width();
        let client_height = self.canvas.client_height();
        let canvas_width = self.canvas.width() as i32;
        let canvas_height = self.canvas.height() as i32;

        if client_width != canvas_width || client_height != canvas_height {
            self.gl.viewport(0, 0, client_width, client_height);
            let client_width = client_width as u32;
            let client_height = client_height as u32;

            self.canvas.set_width(client_width);
            self.canvas.set_height(client_height);
            self.camera.aspect = (client_width as f32) / (client_height as f32);
            self.resolution = (client_width, client_height);

            self.gbuffer.delete(&self.gl);
            self.gbuffer =
                GBuffer::new(&self.gl, self.resolution).expect("Failed to create GBuffer");

            log(&format!("Resized to {}:{}", client_width, client_height));
        }
    }

    pub fn animation_frame(&mut self) {
        self.check_resize();
        let now = window().unwrap().performance().unwrap().now();
        let time = (now / 1000.0) as f32;

        self.render(time);
    }

    fn render(&mut self, time: f32) {
        self.gbuffer.bind(&self.gl);
        self.gl.clear(
            WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT,
        );

        let (world_to_camera, camera_to_screen) = self.camera.to_matrices();

        {
            // Render the background
            self.shader_background
                .setup(&self.gl, world_to_camera, camera_to_screen);
            self.resources
                .meshes
                .quad_quad
                .bind_and_render(&self.gl, &self.shader_background.attributes);
        }

        {
            // Render the models
            self.shader_stl.setup(
                &self.gl,
                &world_to_camera,
                &camera_to_screen,
                &self.resources.png_images.matcap,
                &self.resources.png_images.ship_tex,
            );

            {
                // Ship
                let mut ship_position =
                    Mat4::from_translation(Vec3::new(f32::sin(time) + 3.0, 0.0, 0.0));
                ship_position =
                    ship_position * Mat4::from_rotation_ypr(-0.3 + f32::cos(time) * 0.2, 0.0, time);
                let ship_color = Vec4::new(1.0, 1.0, 1.0, 1.0);
                //~ let glass_color = Vec4::new(0.2, 0.2, 0.6, 0.2);

                let ship_entities = vec![
                    &self.resources.meshes.vehicle_dashboard,
                    &self.resources.meshes.vehicle_cockpit_frame,
                    &self.resources.meshes.vehicle_overhead_panel,
                    &self.resources.meshes.vehicle_chassis,
                ];
                self.shader_stl
                    .set_entity_data(&self.gl, ship_position, ship_color);

                for ship_entity in ship_entities {
                    ship_entity.bind_and_render(&self.gl, &self.shader_stl.attributes);
                }

                //~ self.shader_stl.set_entity_data(
                //~ &self.gl,
                //~ ship_position,
                //~ glass_color,
                //~ );
                //~ self.resources
                //~ .meshes
                //~ .vehicle_glass
                //~ .bind_and_render(&self.gl, &self.shader_stl.attributes);
            }

            // Set up for the other assets
            self.shader_stl.setup(
                &self.gl,
                &world_to_camera,
                &camera_to_screen,
                &self.resources.png_images.matcap,
                &self.resources.png_images.other_assets,
            );

            {
                // Other objects dotted around
                self.shader_stl.set_entity_data(
                    &self.gl,
                    Mat4::from_translation(Vec3::new(0.0, 0.0, 2.0)),
                    Vec4::new(1.0, 1.0, 1.0, 1.0),
                );
                self.resources
                    .meshes
                    .other_assets_landing_pad
                    .bind_and_render(&self.gl, &self.shader_stl.attributes);

                self.shader_stl.set_entity_data(
                    &self.gl,
                    Mat4::from_translation(Vec3::new(10.0, 0.0, 2.0)),
                    Vec4::new(1.0, 1.0, 1.0, 1.0),
                );
                self.resources
                    .meshes
                    .other_assets_light_truss
                    .bind_and_render(&self.gl, &self.shader_stl.attributes);

                self.shader_stl.set_entity_data(
                    &self.gl,
                    Mat4::from_translation(Vec3::new(10.0, 10.0, -2.0)),
                    Vec4::new(1.0, 1.0, 1.0, 1.0),
                );
                self.resources
                    .meshes
                    .other_assets_fuel_tank
                    .bind_and_render(&self.gl, &self.shader_stl.attributes);
                self.shader_stl.set_entity_data(
                    &self.gl,
                    Mat4::from_translation(Vec3::new(7.0, 18.0, -2.0)),
                    Vec4::new(1.0, 1.0, 1.0, 1.0),
                );
                self.resources
                    .meshes
                    .other_assets_fuel_tank
                    .bind_and_render(&self.gl, &self.shader_stl.attributes);
            }
        }

        self.gl
            .bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, None);
        self.shader_lighting_pass.setup(
            &self.gl,
            &self.gbuffer.normal_depth_target,
            &self.gbuffer.albedo_target,
            &self.resources.png_images.matcap,
        );

        self.resources
            .meshes
            .quad_quad
            .bind_and_render(&self.gl, &self.shader_lighting_pass.attributes);
    }

    pub fn mouse_move(&mut self, event: MouseEvent) {
        const DRAG_SENSITIVITY: f32 = 5.0;
        match self.click_location {
            Some(location) => {
                let new = (event.client_x(), event.client_y());
                let delta = (location.0 - new.0, location.1 - new.1);
                self.click_location = Some(new);

                let percentage_x = (delta.0 as f32) / (self.resolution.0 as f32) * DRAG_SENSITIVITY;
                let percentage_y = (delta.1 as f32) / (self.resolution.0 as f32) * DRAG_SENSITIVITY;

                self.camera.azimuth += percentage_x;
                self.camera.elevation -= percentage_y;
                self.camera.elevation = f32::min(f32::max(self.camera.elevation, -1.4), 1.4);
            }
            None => {}
        }
    }
    pub fn mouse_down(&mut self, event: MouseEvent) {
        self.click_location = Some((event.client_x(), event.client_y()));
    }
    pub fn mouse_up(&mut self, _event: MouseEvent) {
        self.click_location = None;
    }

    pub fn key_event(&mut self, event: KeyEvent) {
        log(&format!("Key Event {:?}", event));
    }
}

fn get_gl_context(canvas: &HtmlCanvasElement) -> Result<WebGl2RenderingContext, JsValue> {
    Ok(canvas.get_context("webgl2")?.unwrap().dyn_into()?)
}