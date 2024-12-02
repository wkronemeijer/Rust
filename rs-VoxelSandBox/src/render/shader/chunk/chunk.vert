#version 330

in vec3 pos;
in vec2 tex;

uniform mat4 mvp;

out vec2 uv;

void main() {
    gl_Position = mvp * vec4(pos, 1.0);
    uv = tex;
}
