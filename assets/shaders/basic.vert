#version 330 core

layout (location = 0) in vec3 inPos;
layout (location = 1) in vec3 inColor;
layout (location = 2) in vec2 inUV;

out vec2 aUV;
out vec3 aColor;

void main()
{
    gl_Position = vec4(inPos, 1.0);
    aColor = inColor;
    aUV = inUV;
}