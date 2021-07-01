#version 450 core

layout(location = 0) in vec3 pos;
layout(location = 1) in vec3 color;

layout(location = 0) out vec3 fragColor;

void main(void) {
    gl_Position = vec4(pos.xyz, 1.0);
    fragColor = color;
}