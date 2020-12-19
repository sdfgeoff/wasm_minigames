pub struct VertexShaders {
    pub background: web_sys::WebGlShader,
    pub model: web_sys::WebGlShader,
}

impl VertexShaders {
    fn new(gl: &web_sys::WebGl2RenderingContext) -> Result<Self, crate::shader::ShaderError> {
        Ok(Self {
            background: crate::shader::load_shader(gl, web_sys::WebGl2RenderingContext::VERTEX_SHADER, include_str!("raw/background.vert"))?,
            model: crate::shader::load_shader(gl, web_sys::WebGl2RenderingContext::VERTEX_SHADER, include_str!("raw/model.vert"))?,
        })
    }
}

pub struct FragmentShaders {
    pub background: web_sys::WebGlShader,
    pub model: web_sys::WebGlShader,
}

impl FragmentShaders {
    fn new(gl: &web_sys::WebGl2RenderingContext) -> Result<Self, crate::shader::ShaderError> {
        Ok(Self {
            background: crate::shader::load_shader(gl, web_sys::WebGl2RenderingContext::FRAGMENT_SHADER, include_str!("raw/background.frag"))?,
            model: crate::shader::load_shader(gl, web_sys::WebGl2RenderingContext::FRAGMENT_SHADER, include_str!("raw/model.frag"))?,
        })
    }
}

pub struct PngImages {
    pub matcap: web_sys::WebGlTexture,
    pub ship_tex: web_sys::WebGlTexture,
    pub other_assets: web_sys::WebGlTexture,
}

impl PngImages {
    fn new(gl: &web_sys::WebGl2RenderingContext) -> Result<Self, crate::texture::TextureError> {
        Ok(Self {
            matcap: crate::texture::load_texture(gl, include_bytes!("raw/matcap.png"))?,
            ship_tex: crate::texture::load_texture(gl, include_bytes!("raw/ship_tex.png"))?,
            other_assets: crate::texture::load_texture(gl, include_bytes!("raw/other_assets.png"))?,
        })
    }
}

pub struct Meshes {
    pub quad_quad: crate::geometry::Geometry,
    pub vehicle_chassis: crate::geometry::Geometry,
    pub vehicle_cockpit_frame: crate::geometry::Geometry,
    pub vehicle_glass: crate::geometry::Geometry,
    pub vehicle_overhead_panel: crate::geometry::Geometry,
    pub vehicle_dashboard: crate::geometry::Geometry,
    pub other_assets_fuel_tank: crate::geometry::Geometry,
    pub other_assets_landing_pad: crate::geometry::Geometry,
    pub other_assets_light_truss: crate::geometry::Geometry,
}

impl Meshes {
    fn new(gl: &web_sys::WebGl2RenderingContext) -> Result<Self, crate::geometry::GeometryError> {
        Ok(Self {
            quad_quad: crate::mesh::load_mesh(gl, include_bytes!("raw/quad_quad.mesh"))?,
            vehicle_chassis: crate::mesh::load_mesh(gl, include_bytes!("raw/vehicle_chassis.mesh"))?,
            vehicle_cockpit_frame: crate::mesh::load_mesh(gl, include_bytes!("raw/vehicle_cockpit_frame.mesh"))?,
            vehicle_glass: crate::mesh::load_mesh(gl, include_bytes!("raw/vehicle_glass.mesh"))?,
            vehicle_overhead_panel: crate::mesh::load_mesh(gl, include_bytes!("raw/vehicle_overhead_panel.mesh"))?,
            vehicle_dashboard: crate::mesh::load_mesh(gl, include_bytes!("raw/vehicle_dashboard.mesh"))?,
            other_assets_fuel_tank: crate::mesh::load_mesh(gl, include_bytes!("raw/other_assets_fuel_tank.mesh"))?,
            other_assets_landing_pad: crate::mesh::load_mesh(gl, include_bytes!("raw/other_assets_landing_pad.mesh"))?,
            other_assets_light_truss: crate::mesh::load_mesh(gl, include_bytes!("raw/other_assets_light_truss.mesh"))?,
        })
    }
}

impl Resources {
    pub fn new(gl: &web_sys::WebGl2RenderingContext) -> Result<Self, Box<dyn std::error::Error>> {
        let vertex_shaders = VertexShaders::new(gl)?;
        let fragment_shaders = FragmentShaders::new(gl)?;
        let png_images = PngImages::new(gl)?;
        let meshes = Meshes::new(gl)?;
        Ok(Self {
            vertex_shaders,
            fragment_shaders,
            png_images,
            meshes,
        })
    }
}

pub struct Resources {
    pub vertex_shaders: VertexShaders,
    pub fragment_shaders: FragmentShaders,
    pub png_images: PngImages,
    pub meshes: Meshes,
}