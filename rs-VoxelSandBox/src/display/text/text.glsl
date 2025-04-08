////////////////////
// @vertex shader //
////////////////////

#version 330

in vec2 pos;
in vec2 tex;

out vec2 uv;

// Projection will contain a device-coordinate ortho
uniform mat4 projection;

void main() {
    gl_Position = projection * vec4(pos, 0.0, 1.0);
    uv = tex;
}

//////////////////////
// @fragment shader //
//////////////////////

#version 330
precision mediump float;

in vec2 uv;

out vec4 outputColor;

uniform sampler2D font;
uniform vec4 color;
uniform vec4 background;

void main() {
    // For font.png:
    // - foreground is (1.0, 1.0, 1.0)
    // - background is (1.0, 0.0, 1.0)
    float sigma = texture(font, uv).g; 
    vec4 c = mix(background, color, sigma);
    if (c.a < 0.95) {
        discard;
    }
    outputColor = c;
}
