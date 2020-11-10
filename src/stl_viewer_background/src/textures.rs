// Contains static textures

use super::texture::{load_texture, TextureError};
use web_sys::{WebGl2RenderingContext, WebGlTexture};

pub struct StaticTextures {
    pub stl_matcap: WebGlTexture,
}

impl StaticTextures {
    pub fn new(gl: &WebGl2RenderingContext) -> Result<Self, TextureError> {
        
        let stl_matcap = load_texture(&gl, include_bytes!("resources/matcap.png"))?;
        
        Ok(Self {
            stl_matcap: stl_matcap,
        })
    }
}
