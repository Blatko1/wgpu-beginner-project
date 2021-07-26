#version 450 core

layout(location = 0) in vec3 pos;
layout(location = 1) in vec2 tex_cords;
layout(location = 2) in vec3 color;
layout(location = 3) in vec3 normal;

layout(location = 4) in vec4 model_mat1;
layout(location = 5) in vec4 model_mat2;
layout(location = 6) in vec4 model_mat3;
layout(location = 7) in vec4 model_mat4;

layout(location = 8) in vec3 n_matrix1;
layout(location = 9) in vec3 n_matrix2;
layout(location = 10) in vec3 n_matrix3;

/*layout(location = 11) in vec3 ambient_color;
layout(location = 12) in vec3 diffuse_color;
layout(location = 13) in vec3 specular_color;
layout(location = 14) in float shinines;
layout(location = 15) in float include_tex;
layout(location = 16) in float include_diff_color;*/

layout(set = 0, binding = 0) uniform matrixUniform {
    mat4 proj_view_model_matrix;
    vec3 view_pos;
};

layout(location = 0) out vec3 fragColor;
layout(location = 1) out vec2 frag_tex_cords;
layout(location = 2) out vec3 v_pos;
layout(location = 3) out vec3 v_normal;

void main(void) {
    mat4 model_matrix = mat4(model_mat1, model_mat2, model_mat3, model_mat4);
    mat3 n_matrix = mat3(n_matrix1, n_matrix2, n_matrix3);
    vec4 world_position = model_matrix * vec4(pos.xyz, 1.0);
    gl_Position = proj_view_model_matrix * world_position;
    fragColor = color;
    frag_tex_cords = tex_cords;
    v_pos = world_position.xyz;
    v_normal = n_matrix * normal;
}