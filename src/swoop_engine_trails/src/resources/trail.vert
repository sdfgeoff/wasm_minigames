#version 300 es

precision mediump float;
in vec4 aVertexPosition;

uniform mat3 world_to_camera;
uniform mat3 world_to_sprite;
uniform mat3 camera_to_clipspace; // Includes canvas resolution/aspect ratio

const float trail_width = 0.1;
uniform vec4 point_buffer[20]; // Note: webgl specs require a min of 128 vec4 uniforms, so this could be extended
uniform vec4 data_buffer[20]; // Note: webgl specs require a min of 128 vec4 uniforms, so this could be extended
uniform int point_buffer_length;
uniform float trail_percent_offset;

out vec2 uv;
out vec4 data;
out float segment_percent;
out float trail_percent;

void main() {
    mat3 camera_to_world = inverse(world_to_camera);
    mat3 clipspace_to_camera = inverse(camera_to_clipspace);
    mat3 camera_to_sprite = camera_to_world * world_to_sprite;
    mat3 sprite_to_clipspace = clipspace_to_camera * camera_to_sprite;
    
    uv = aVertexPosition.xy;


    float vert_id_raw = aVertexPosition.y * (float(point_buffer_length) - 1.0);
    float segment = floor(vert_id_raw);
    int index_here = int(segment);

    // Find a bunch of points in all directions
    vec4 p1 = point_buffer[index_here];
    vec4 p2 = point_buffer[index_here+1];
    vec4 d1 = data_buffer[index_here];
    vec4 d2 = data_buffer[index_here+1];

    segment_percent = mod(vert_id_raw, 1.0);
    data = mix(d1, d2, segment_percent);

    float t = segment_percent;
    float t2 = t * t;
    float t3 = t2 * t;

    // Cubic Hermite Interpolation
    float H0 = 2.0 * t3 - 3.0 * t2 + 1.0;
    float H1 = -2.0 * t3 + 3.0 * t2;
    float H2 = t3 - 2.0 * t2 + t;
    float H3 = t3 - t2;

    vec2 h0 = p1.xy; // Position
    vec2 h1 = p2.xy;
    vec2 t0 = p1.zw; // Tangents
    vec2 t1 = p2.zw;

    if (index_here == 0) {
        // Prevent interpolation in front of the ship
        // This is caused by the h0 and h1 being in very similar positions
        // and t1 causes the curve to precede.
        t1 = normalize(t1) * length(t0);
    }


    vec2 centerline = h0 * H0 + h1 * H1 + t0 * H2 + t1 * H3;

    vec2 tangent = normalize(mix(t0, t1, segment_percent));
    vec2 normal = vec2(tangent.y, - tangent.x) * trail_width;
    vec2 offset = normal * aVertexPosition.x;

    vec2 vert_position = centerline + offset;
    
    trail_percent = uv.y - trail_percent_offset;

    //vert_position = uv + point_buffer[0].xy;
    vec2 pos = (sprite_to_clipspace * vec3(vert_position, 1.0)).xy;

    
    
    gl_Position = vec4(pos, 0.0, 1.0);
}
