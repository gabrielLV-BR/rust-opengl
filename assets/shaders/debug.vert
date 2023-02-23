#version 330 core

layout(location=0) in vec3 inPos;
layout(location=1) in vec3 inNor;
layout(location=2) in vec2 inUV;

uniform mat4 aModelMatrix;

out vec3 aNor;
out vec2 aUV;

void main() {
    gl_Position = aModelMatrix * vec4(inPos, 1.0);
    aNor = inNor;
    aUV = inUV;
}