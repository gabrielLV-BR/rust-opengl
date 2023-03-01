#version 330 core

layout(location=0) in vec3 inPos;
layout(location=1) in vec2 inUV;

// uniform mat4 aModelMatrix;

out vec2 aUV;

void main() {
    gl_Position = vec4(inPos, 1.0);
    aUV = inUV;
}