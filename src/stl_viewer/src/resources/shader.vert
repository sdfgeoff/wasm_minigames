#version 300 es

precision mediump float;
in vec4 vert_pos;
in vec4 vert_nor;

uniform vec2 iResolution;

uniform float iTime;

out vec4 screen_pos;
out vec4 screen_nor;


mat4 rot_y(float angle) {
        float c = cos(angle);
        float s = sin(angle);
        return mat4(
                vec4(c, 0.0, s, 0.0),
                vec4(0.0, 1.0, 0.0, 0.0),
                vec4(-s, 0.0, c, 0.0),
                vec4(0.0, 0.0, 0.0, 1.0)
        );
}

mat4 rot_x(float angle) {
        float c = cos(angle);
        float s = sin(angle);
        return mat4(
                vec4(1.0, 0.0, 0.0, 0.0),
                vec4(0.0, c, s, 0.0),
                vec4(0.0, -s, c, 0.0),
                vec4(0.0, 0.0, 0.0, 1.0)
        );
}



void main() {
        
        mat4 pan = rot_y(iTime);
        mat4 tilt = rot_x(sin(iTime));
        
        mat4 mat = tilt * pan;
        
	screen_pos = mat * vert_pos;
        screen_nor = mat * vert_nor;
	
        gl_Position = screen_pos;
        gl_Position.x *= iResolution.y / iResolution.x;
        gl_Position.w = 1.0;
}
