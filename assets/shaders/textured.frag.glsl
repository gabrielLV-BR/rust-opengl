#version 330 core

uniform sampler2D uTexture;

out vec4 outColor;
in vec2 aUV;

void main() {
    // outColor = vec4(aUV, 0.0, 1.0);
    outColor = texture(uTexture, aUV);
}