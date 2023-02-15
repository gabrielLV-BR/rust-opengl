#version 330 core

layout (location = 0) in vec3 inPos;
layout (location = 1) in vec3 inColor;
layout (location = 2) in vec2 inUV;

out vec2 aUV;
out vec3 aColor;

uniform mat4 uModelMatrix;
uniform mat4 uViewMatrix;
uniform mat4 uProjMatrix;

void main()
{
    gl_Position = uProjMatrix * uViewMatrix * uModelMatrix * vec4(inPos, 1.0);
    aColor = inColor;
    aUV = inUV;
}