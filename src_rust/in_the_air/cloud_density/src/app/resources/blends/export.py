import os

export_single = r'''
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
    uv0 = []
    
    dedup_data_lookup = {}
    
    for loop_tri in mesh.loop_triangles:
        triangle_indices = []
        
        for loop_index in loop_tri.loops:
            loop = mesh.loops[loop_index]

            vert = mesh.vertices[loop.vertex_index]
            position = tuple(vert.co)
            normal = tuple(loop.normal)
            
            if mesh.uv_layers:
                uv = tuple(mesh.uv_layers[0].data[loop_index].uv)
            else:
                uv = (0.0, 0.0)

            dedup = (position, normal, uv)
            if dedup not in dedup_data_lookup:
                index = len(verts)
                verts.append(position)
                normals.append(normal)
                uv0.append(uv)
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
    for uv in uv0:
        out_data += struct.pack("ff", *uv)
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
'''

EXPORT_FILE_PATH = "/tmp/export_blend.py"
open(EXPORT_FILE_PATH, 'w').write(export_single)


def export_blend(filepath):
    os.system("blender -b {} --python {}".format(filepath, EXPORT_FILE_PATH))

current_dir = os.path.dirname(os.path.abspath(__file__))
output_dir = os.path.join(current_dir, '..')

for filename in os.listdir(output_dir):
    if filename.endswith(".mesh"):
        full_path = os.path.join(output_dir, filename)
        os.remove(full_path)

for filename in os.listdir():
    if filename.endswith(".blend"):
        export_blend(os.path.join(current_dir, filename))
