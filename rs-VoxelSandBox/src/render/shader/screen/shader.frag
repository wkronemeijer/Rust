#version 330

in vec2 uv;

uniform sampler2D tex;

out vec4 outputColor;

void main() {
    outputColor = texture(tex, uv);
}
