#version 330 core

layout (location = 0) in vec3 inPosition;
// layout (location = 1) in vec3 inNormal;
// layout (location = 2) in vec2 inUV;

// out vec2 aUV;

// uniform mat4 uModelMatrix;
// uniform mat4 uViewMatrix;
// uniform mat4 uProjMatrix;

void main()
{
    // gl_Position = uProjMatrix * uViewMatrix * uModelMatrix * vec4(inPosition, 1.0);
    gl_Position = vec4(inPosition, 1.0);
    // aUV = inUV;
}