////////////////////
// @vertex shader //
////////////////////

#version 330

in vec2 pos;
in vec2 tex;

out vec2 uv;

void main() {
    gl_Position = vec4(pos, 0.0, 1.0);
    uv = tex;
}

//////////////////////
// @fragment shader //
//////////////////////

#version 330
precision mediump float;

in vec2 uv;

uniform sampler2D tex;

out vec4 outputColor;

void main() {
    outputColor = texture(tex, uv);
}
