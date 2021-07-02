#version 450 core

layout(location = 0) in vec3 pos;
layout(location = 1) in vec3 color;

layout(set = 0, binding = 0) uniform matrixUniform {
    mat4 proj_view_model_matrix;
};

layout(location = 0) out vec3 fragColor;

void main(void) {
    gl_Position = proj_view_model_matrix * vec4(pos.xyz, 1.0);
    fragColor = color;
}