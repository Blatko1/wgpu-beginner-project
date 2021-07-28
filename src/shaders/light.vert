#version 450 core

layout(location = 0) in vec3 pos;

layout(location = 0) out vec3 v_color;

layout(location = 3) in vec4 model_mat1;
layout(location = 4) in vec4 model_mat2;
layout(location = 5) in vec4 model_mat3;
layout(location = 6) in vec4 model_mat4;

layout(set = 0, binding = 0) uniform matrixUniform {
    mat4 proj_view_model_matrix;
    vec3 view_pos;
};

layout(set = 1, binding = 0)
uniform Light {
    vec3 u_position;
    vec3 u_color;
};

float scale = 0.25;

void main(void) {
    vec3 l_pos = scale * pos + u_position;
    gl_Position = proj_view_model_matrix * vec4(l_pos, 1.0);
    v_color = u_color;
}