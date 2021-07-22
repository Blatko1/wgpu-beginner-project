#version 450 core

layout(location = 0) in vec3 pos;
layout(location = 1) in vec2 tex_cords;
layout(location = 2) in vec3 color;
layout(location = 3) in vec3 normals;

layout(location = 4) in vec4 model_mat1;
layout(location = 5) in vec4 model_mat2;
layout(location = 6) in vec4 model_mat3;
layout(location = 7) in vec4 model_mat4;

layout(set = 0, binding = 0) uniform matrixUniform {
    mat4 proj_view_model_matrix;
};

layout(location = 0) out vec3 fragColor;
layout(location = 1) out vec2 frag_tex_cords;

void main(void) {
    mat4 model_matrix = mat4(model_mat1, model_mat2, model_mat3, model_mat4);
    gl_Position = proj_view_model_matrix * model_matrix * vec4(pos.xyz, 1.0);
    fragColor = color;
    frag_tex_cords = tex_cords;
}