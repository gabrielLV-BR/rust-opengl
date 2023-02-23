#version 330 core

out vec4 outColor;

in vec3 aNor;
in vec2 aUV;

void main() {
    outColor = vec4(aNor, 1.0);
}