#version 330

in vec2 position;
in vec2 tex_coords;

out vec2 uv;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    uv = tex_coords;
}
