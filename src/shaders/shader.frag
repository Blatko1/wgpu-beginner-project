#version 450 core

layout(location = 0) in vec3 fragColor;

layout(location = 0) out vec4 outColor;

void main(void) {
    outColor = vec4(fragColor.xyz, 1.0);
}