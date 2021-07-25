#version 450 core

layout(location = 0) in vec3 v_color;
layout(location = 0) out vec4 f_color;

void main(void) {
    f_color = vec4(v_color, 1.0);
}