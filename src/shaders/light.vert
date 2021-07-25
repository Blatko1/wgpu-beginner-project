#version 450 core

layout(location = 0) in vec3 pos;

layout(location = 0) out vec3 v_color;

layout(location = 4) in vec4 model_mat1;
layout(location = 5) in vec4 model_mat2;
layout(location = 6) in vec4 model_mat3;
layout(location = 7) in vec4 model_mat4;

layout(set = 0, binding = 0) uniform matrixUniform {
    mat4 proj_view_model_matrix;
};

layout(set = 1, binding = 0)
uniform Light {
    vec3 u_position;
    vec3 padding1;
    vec2 padding2;
    vec3 u_color;
};

void main(void) {
    gl_Position = proj_view_model_matrix * vec4(pos.x, pos.y + 5.0, pos.z, 1.0);
    v_color = padding1;
}