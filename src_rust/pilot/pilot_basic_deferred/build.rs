/// This file generates a "resources.rs" file that contains the struct
/// "Resources". When this struct is created, it contains all the
/// resouces for the game (eg meshes, textures, shaders) already
/// uploaded to the GPU.
use std::fs::{read_dir, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use std::collections::HashMap;

use codegen::{Function, Impl, Scope, Struct};

const RESOURCES_FOLDER: &'static str = "src/resources/raw";
const OUT_FILE: &'static str = "src/resources/mod.rs";

/// Create a struct with a field for each vertex shader and an initializer
/// that loads the shader to the GPU.
fn generate_vertex_shaders(out_scope: &mut Scope, files: &Vec<PathBuf>) {
    let mut vert_struct = Struct::new("VertexShaders");
    vert_struct.vis("pub");

    let mut vert_struct_initilizer = Function::new("new");
    vert_struct_initilizer.ret("Result<Self, crate::shader::ShaderError>");
    vert_struct_initilizer.arg("gl", "&web_sys::WebGl2RenderingContext");

    vert_struct_initilizer.line("Ok(Self {");

    for file in files {
        let field_name = file.file_stem().unwrap().to_str().unwrap();

        vert_struct_initilizer.line(format!(
            "    {}: crate::shader::load_shader(gl, web_sys::WebGl2RenderingContext::VERTEX_SHADER, include_str!(\"{}\"))?,",
            field_name,
            file.to_str().unwrap(),
        ));
        vert_struct.field(&format!("pub {}", field_name), "web_sys::WebGlShader");
    }

    vert_struct_initilizer.line("})");

    let mut vert_struct_impl = Impl::new(vert_struct.ty());
    vert_struct_impl.push_fn(vert_struct_initilizer);

    out_scope.push_struct(vert_struct);
    out_scope.push_impl(vert_struct_impl);
}

/// Create a struct with a field for each fragment shader and an initializer
/// that loads the shader to the GPU.
fn generate_fragment_shaders(out_scope: &mut Scope, files: &Vec<PathBuf>) {
    let mut frag_struct = Struct::new("FragmentShaders");
    frag_struct.vis("pub");

    let mut frag_struct_initilizer = Function::new("new");
    frag_struct_initilizer.ret("Result<Self, crate::shader::ShaderError>");
    frag_struct_initilizer.arg("gl", "&web_sys::WebGl2RenderingContext");

    frag_struct_initilizer.line("Ok(Self {");

    for file in files {
        let field_name = file.file_stem().unwrap().to_str().unwrap();

        frag_struct_initilizer.line(format!(
            "    {}: crate::shader::load_shader(gl, web_sys::WebGl2RenderingContext::FRAGMENT_SHADER, include_str!(\"{}\"))?,",
            field_name,
            file.to_str().unwrap(),
        ));
        frag_struct.field(&format!("pub {}", field_name), "web_sys::WebGlShader");
    }

    frag_struct_initilizer.line("})");

    let mut frag_struct_impl = Impl::new(frag_struct.ty());
    frag_struct_impl.push_fn(frag_struct_initilizer);

    out_scope.push_struct(frag_struct);
    out_scope.push_impl(frag_struct_impl);
}

/// Create a struct for all images and an initilizer that loads them to
/// the GPU.
fn generate_png_images(out_scope: &mut Scope, files: &Vec<PathBuf>) {
    let mut png_struct = Struct::new("PngImages");
    png_struct.vis("pub");

    let mut png_struct_initilizer = Function::new("new");
    png_struct_initilizer.ret("Result<Self, crate::texture::TextureError>");
    png_struct_initilizer.arg("gl", "&web_sys::WebGl2RenderingContext");

    png_struct_initilizer.line("Ok(Self {");

    for file in files {
        let field_name = file.file_stem().unwrap().to_str().unwrap();

        png_struct_initilizer.line(format!(
            "    {}: crate::texture::load_texture(gl, include_bytes!(\"{}\"))?,",
            field_name,
            file.to_str().unwrap(),
        ));
        png_struct.field(
            &format!("pub {}", field_name),
            "Option<web_sys::WebGlTexture>",
        );
    }

    png_struct_initilizer.line("})");

    let mut png_struct_impl = Impl::new(png_struct.ty());
    png_struct_impl.push_fn(png_struct_initilizer);

    out_scope.push_struct(png_struct);
    out_scope.push_impl(png_struct_impl);
}

fn generate_meshes(out_scope: &mut Scope, files: &Vec<PathBuf>) {
    let mut mesh_struct = Struct::new("Meshes");
    mesh_struct.vis("pub");

    let mut mesh_struct_initilizer = Function::new("new");
    mesh_struct_initilizer.ret("Result<Self, crate::geometry::GeometryError>");
    mesh_struct_initilizer.arg("gl", "&web_sys::WebGl2RenderingContext");

    mesh_struct_initilizer.line("Ok(Self {");

    for file in files {
        let field_name = file.file_stem().unwrap().to_str().unwrap();

        mesh_struct_initilizer.line(format!(
            "    {}: crate::mesh::load_mesh(gl, include_bytes!(\"{}\"))?,",
            field_name,
            file.to_str().unwrap(),
        ));
        mesh_struct.field(&format!("pub {}", field_name), "crate::geometry::Geometry");
    }

    mesh_struct_initilizer.line("})");

    let mut mesh_struct_impl = Impl::new(mesh_struct.ty());
    mesh_struct_impl.push_fn(mesh_struct_initilizer);

    out_scope.push_struct(mesh_struct);
    out_scope.push_impl(mesh_struct_impl);
}

fn main() {
    let resources_path = Path::new(RESOURCES_FOLDER);

    let mut resources: HashMap<String, Vec<PathBuf>> = HashMap::new();

    for file in read_dir(resources_path).unwrap() {
        let file = file.unwrap();

        let filetype = file.file_type().unwrap();

        if filetype.is_file() {
            let filepath = file.path();
            let fa = filepath.clone();
            let extension = fa.extension();

            let extension_string: String = extension.unwrap().to_os_string().into_string().unwrap();

            if !resources.contains_key(&extension_string) {
                resources.insert(extension_string.clone(), Vec::new());
            }

            let fb = filepath.clone();
            let prefix = fb.strip_prefix("src/resources");
            let prefix = prefix.unwrap();

            resources
                .get_mut(&extension_string)
                .unwrap()
                .push(prefix.to_path_buf());
        }
    }

    let mut out_file = Scope::new();

    let mut resources_struct = Struct::new("Resources");
    resources_struct.vis("pub");
    let mut resources_struct_impl = Impl::new(resources_struct.ty());

    let mut resources_struct_initilizer = Function::new("new");
    resources_struct_initilizer.vis("pub");
    resources_struct_initilizer.ret("Result<Self, Box<dyn std::error::Error>>");
    resources_struct_initilizer.arg("gl", "&web_sys::WebGl2RenderingContext");

    let mut fields = vec![];

    if let Some(values) = resources.get("vert") {
        generate_vertex_shaders(&mut out_file, values);
        fields.push(("vertex_shaders", "VertexShaders"));
        resources_struct_initilizer.line("let vertex_shaders = VertexShaders::new(gl)?;");
    }

    if let Some(values) = resources.get("frag") {
        generate_fragment_shaders(&mut out_file, values);
        fields.push(("fragment_shaders", "FragmentShaders"));
        //resources_struct.field("pub fragment_shaders", "FragmentShaders");
        resources_struct_initilizer.line("let fragment_shaders = FragmentShaders::new(gl)?;");
    }

    if let Some(values) = resources.get("png") {
        generate_png_images(&mut out_file, values);
        fields.push(("png_images", "PngImages"));
        resources_struct_initilizer.line("let png_images = PngImages::new(gl)?;");
    }
    //~ if let Some(values) = resources.get("stl") {
    //~ generate_stl_meshes(&mut out_file, values);
    //~ fields.push(("stl_meshes", "StlMeshes"));
    //~ resources_struct_initilizer.line("let stl_meshes = StlMeshes::new(gl)?;");
    //~ }
    if let Some(values) = resources.get("mesh") {
        generate_meshes(&mut out_file, values);
        fields.push(("meshes", "Meshes"));
        resources_struct_initilizer.line("let meshes = Meshes::new(gl)?;");
    }
    //~ if let Some(values) = resources.get("prog") {
    //~ generate_shader_programs(&mut out_file, values);
    //~ fields.push(("shader_programs", "ShaderPrograms"));
    //~ resources_struct_initilizer.line("let shader_programs = ShaderPrograms::new(gl, &vertex_shaders, &fragment_shaders)?;");
    //~ }

    resources_struct_initilizer.line("Ok(Self {");
    for (field_name, type_name) in fields {
        resources_struct.field(&format!("pub {}", field_name), type_name);
        resources_struct_initilizer.line(&format!("    {},", field_name));
    }
    resources_struct_initilizer.line("})");
    resources_struct_impl.push_fn(resources_struct_initilizer);

    out_file.push_impl(resources_struct_impl);
    out_file.push_struct(resources_struct);

    let path = Path::new(OUT_FILE);
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(out_file.to_string().as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }

    println!("cargo:rerun-if-changed={}", RESOURCES_FOLDER);
}
