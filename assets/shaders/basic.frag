#version 330 core

// in vec2 aUV;
out vec4 outColor;

// uniform sampler2D uDiffuseMap;
// uniform sampler2D uNormalMap;
uniform vec3 uColor;

void main()
{
    //outColor = vec4(aUV, 1.0, 1.0);
    // outColor = texture(uTexture, aUV);
    outColor = vec4(uColor, 1.0);
} 