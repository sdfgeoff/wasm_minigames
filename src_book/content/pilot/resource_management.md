# Resource Management

Up to now we've only had a few resources: one or two sprites, one or
two meshes. To make a 3D game, we're going to have a lot more. It's 
time to do some automation and some refactoring.

Let's start with our STL viewer. Currently it has the textures stored in
a "StaticTextures" struct that many things can access. Let's do something
similar to meshes. Heck, lets do something similar to shaders as well.
Let's call this struct `Resources`. It can look something like:
```rust
struct Resources{
    images: Images
    meshes: Meshes
    shaders: Shaders
}

struct Images: {
    matcap: WebGlTexture
    background: WebGlTexture
}

struct Meshes {
    some_mesh_name: SomeSortOfMesh,
    other_mesh_name: SomeSortOfMesh,
}
...
```

Doing this allows rust to statically check that we have all the required
resources, but creating this resources struct by hand would be tiresome.
Fortunately cargo can run build scripts, so we can auto-generate a
directory listing with bunches of `SomeStruct::new(gl, include_bytes!(filename)`.

This turned out to harder than I thought and turned into a lot of usage
of the codegen crate, but eventually a solution was made. 
I'm just going to say "check the source".

Another complication was around storage and rendering of meshes. In the 
STL viewer there were two meshes with two shaders. They were different 
enough that they each had their own struct and they each passed data 
from one to the other in a unique way. Fine for just two, but not scalable
to many. These "common bits" that both need to know are the vertex attributes
 - they link the shaders to the geometry.
So it makes sense that we can break them out:
```rust
/// The renderer needs to know where certain things are within the
/// shader so that the geometry can be rendered
pub struct VertexAttributes {
    pub positions: u32,
    pub normals: u32,
}

impl VertexAttributes {
    pub fn new(gl: &WebGl2RenderingContext, program: &WebGlProgram) -> Self {
        Self {
            positions: gl.get_attrib_location(&program, "vert_pos") as u32,
            normals: gl.get_attrib_location(&program, "vert_nor") as u32,
        }
    }
}
```

Similarly, we can unify the storage of meshes by putting everything
into a container:
```rust
pub struct BufferDef {
    /// Reference to the data on the GPU
    buffer: WebGlBuffer,
    
    /// How many components are there in the buffer
    buffer_length: i32
}


pub struct Geometry {
    positions: BufferDef,
    normals: BufferDef,
    indices: BufferDef,
}
```

This means that we can delete the `Stl` and `Background` structs because
they are now instances of the Geometry struct instantiated inside the
`Resources` struct. And the rendering can be put inside the `Geometry`
struct as:
```rust
    /// Mage sure everything is set up for rendering this geometry
    pub fn bind(&mut self, gl: &WebGl2RenderingContext, vertex_attributes: &VertexAttributes) {
        gl.enable_vertex_attrib_array(vertex_attributes.positions);
        gl.bind_buffer(
            WebGl2RenderingContext::ARRAY_BUFFER,
            Some(&self.positions.buffer),
        );
        gl.vertex_attrib_pointer_with_i32(
            vertex_attributes.positions,
            3, // num components
            WebGl2RenderingContext::FLOAT,
            false, // normalize
            0,     // stride
            0,     // offset
        );

        if vertex_attributes.normals != 0xFFFFFFFF {
            gl.enable_vertex_attrib_array(vertex_attributes.normals);
            gl.bind_buffer(
                WebGl2RenderingContext::ARRAY_BUFFER,
                Some(&self.normals.buffer),
            );
            gl.vertex_attrib_pointer_with_i32(
                vertex_attributes.normals,
                3, // num components
                WebGl2RenderingContext::FLOAT,
                false, // normalize
                0,     // stride
                0,     // offset
            );
        }

        gl.bind_buffer(
            WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&self.indices.buffer),
        );
    }

    /// Actually render this geometry
    pub fn render(&mut self, gl: &WebGl2RenderingContext) {
        gl.draw_elements_with_i32(
            WebGl2RenderingContext::TRIANGLES,
            self.indices.buffer_length,
            WebGl2RenderingContext::UNSIGNED_SHORT,
            0,
        );
    }

    /// Convenience function that executes bind and render. You only
    /// want this if you are only rendering a single instance of this
    /// geometry with this shader. Otherwise you can optimize by
    /// calling `bind` once and `render` lots.
    pub fn bind_and_render(
        &mut self,
        gl: &WebGl2RenderingContext,
        vertex_attributes: &VertexAttributes,
    ) {
        self.bind(gl, vertex_attributes);
        self.render(gl);
    }
```

STL is not a great mesh format. It's always flat shaded and doesn't
share vertex locations making for larger file sizes. It also doesn't
support texture coordinates.
I'm not up for supporting a full scene format like gltf, so let's invent
our own format (I'm going to regret this later aren't I?).
We'll start off the file with a header, the header will contain the
number of vertices and the number of faces as u16's. Then the file will 
include all the vertex positions as 3xf32's, the vertex normals as 
3xf32's and finally the face indices as u16's.
Fairly soon I'll add in texture coordinates, but this should do for 
now.

Exporting from blender is simple enough:
```python
import os
import bpy
import struct

def export_mesh(obj, filepath):
    print("Exporting Mesh {} to {}".format(obj.name, filepath))
    
    depsgraph = bpy.context.view_layer.depsgraph
    depsgraph.update()
    
    eval_object = obj.evaluated_get(depsgraph)
    mesh = eval_object.to_mesh(
        #preserve_all_data_layers=preserve_vertex_groups,
        depsgraph=depsgraph
    )

    mesh.calc_loop_triangles()
    mesh.calc_normals_split()
    
    verts = []
    normals = []
    indices = []
    
    dedup_data_lookup = {}
    
    for loop_tri in mesh.loop_triangles:
        triangle_indices = []
        
        for loop_index in loop_tri.loops:
            loop = mesh.loops[loop_index]

            vert = mesh.vertices[loop.vertex_index]
            position = tuple(vert.co)
            normal = tuple(loop.normal)

            dedup = (position, normal)
            if dedup not in dedup_data_lookup:
                index = len(verts)
                verts.append(position)
                normals.append(normal)
                dedup_data_lookup[dedup] = index
            else:
                index = dedup_data_lookup[dedup]
                
            triangle_indices.append(index)
        indices.append(tuple(triangle_indices))
    
    eval_object.to_mesh_clear()
            
    # Output our file
    # We start off with a header containing data about the file
    out_data = b''
    out_data += struct.pack("H", len(verts))
    out_data += struct.pack("H", len(indices)) 
    
    # We don't need len(normals) because:
    assert len(normals) == len(verts)
    
    # Now we can pack all our data:
    for vert in verts:
        out_data += struct.pack("fff", *vert)
    for normal in normals:
        out_data += struct.pack("fff", *normal)
    for index in indices:
        out_data += struct.pack("HHH", *index)
    
    open(filepath, 'wb').write(out_data)


def export_all():
    blend_name = os.path.basename(bpy.data.filepath.replace(".blend", ""))

    for obj in bpy.context.scene.objects:
        if not obj.hide_render:
            output_filepath = os.path.normpath(bpy.path.abspath("//../{}_{}.mesh".format(
                blend_name,
                obj.name
            )))
        
            if obj.type == "MESH":
                export_mesh(obj, output_filepath)

export_all()
```

And importing to our geometry is also nice and simple:
```rust
pub fn load_mesh(gl: &WebGl2RenderingContext, mesh: &[u8]) -> Result<Geometry, GeometryError> {
    let (faces, positions, normals) = extact_buffers_from_mesh(mesh);
    Geometry::new(gl, &positions, &normals, &faces)
}

/// Reads a f32 from a buffer
fn get_f32(arr: &[u8]) -> f32 {
    f32::from_le_bytes(arr[0..4].try_into().unwrap())
}
/// Reads a u16 from a buffer
fn get_u16(arr: &[u8]) -> u16 {
    u16::from_le_bytes(arr[0..2].try_into().unwrap())
}

/// Converts a slice of u8's into a vec of f32;s
fn parse_f32_array(data: &[u8], num_elements: usize) -> Vec<f32> {
    let mut out_array = Vec::with_capacity(num_elements);
    for i in 0..num_elements {
        out_array.push(get_f32(&data[i*4..]));
    }
    out_array
}
/// Converts a slice of u8's into a vec of u16's
fn parse_u16_array(data: &[u8], num_elements: usize) -> Vec<u16> {
    let mut out_array = Vec::with_capacity(num_elements);
    for i in 0..num_elements {
        out_array.push(get_u16(&data[i*2..]));
    }
    out_array
}

/// Converts the bytes of a binary stl file into a vector of face indices,
/// vertices and vertex normals.
/// Expects correctly formatted STL files
fn extact_buffers_from_mesh(mesh: &[u8]) -> (Vec<u16>, Vec<f32>, Vec<f32>) {
    
    let num_verts = u16::from_le_bytes(mesh[0..2].try_into().unwrap()) as usize;
    let num_faces = u16::from_le_bytes(mesh[2..4].try_into().unwrap()) as usize;

    let verts_start = 4;
    let normals_start = verts_start + num_verts * 4 * 3;
    let indices_start = normals_start + num_verts * 4 * 3;
    
    let vertices = parse_f32_array(&mesh[verts_start..], num_verts*3);
    let normals = parse_f32_array(&mesh[normals_start..], num_verts*3);
    let indices = parse_u16_array(&mesh[indices_start..], num_faces*3);

    (indices, vertices, normals)
}
```


Eh, whatever, let's do the texture mapping now
```python
if mesh.uv_layers:
    uv = tuple(mesh.uv_layers[0].data[loop_index].uv)
else:
    uv = (0.0, 0.0)
```

```rust
let verts_start = 4;
let normals_start = verts_start + num_verts * 4 * 3;
let uv0_start = normals_start + num_verts * 4 * 3;
let indices_start = uv0_start + num_verts * 4 * 2;

let vertices = parse_f32_array(&mesh[verts_start..], num_verts*3);
let normals = parse_f32_array(&mesh[normals_start..], num_verts*3);
let uv0 = parse_f32_array(&mesh[uv0_start..], num_verts*2);
let indices = parse_u16_array(&mesh[indices_start..], num_faces*3);
```

```rust
if vertex_attributes.uv0 != 0xFFFFFFFF {
    gl.enable_vertex_attrib_array(vertex_attributes.uv0);
    gl.bind_buffer(
        WebGl2RenderingContext::ARRAY_BUFFER,
        Some(&self.uv0.buffer),
    );
    gl.vertex_attrib_pointer_with_i32(
        vertex_attributes.uv0,
        2, // num components
        WebGl2RenderingContext::FLOAT,
        false, // normalize
        0,     // stride
        0,     // offset
    );
}
```

```glsl
    vec4 new_col = color * texture(image_albedo, uv0);
    
    vec3 out_col = new_col.rgb;
    out_col = out_col * diffuse;
    out_col += reflection * fresnel * 0.5;
    out_col *= 1.0 - fresnel * 0.5;

    FragColor.rgb = out_col;
    FragColor.a = new_col.a;

```


And voila:


<canvas id="pilot_resource_management"></canvas>

That path the ship is following? Yeah, it's just a bunch of sine waves
plugged together.
The strange lighting on the landing platform? I think that's because the 
background is sampled based on the normal, and for a face poining 
directly along the Z axis it hits the seam in the equirectangular map.

It's worth noting that the binary is now approaching 900kb. Most of 
that is occupied by the ship texture which is ~350kb. With a bit of luck
there won't be too many more textures or models required so it shouldn't
grow too much more. When the size approaches a couple Mb we may have to
somehow indicate the load status of the WASM blob.


