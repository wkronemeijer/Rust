#version 330

in vec3 pos;
in vec2 tex;
in float light;

uniform mat4 mvp;

out vec2 uv;
out float l;

void main() {
    gl_Position = mvp * vec4(pos, 1.0);
    uv = tex;
    l = light;
}
