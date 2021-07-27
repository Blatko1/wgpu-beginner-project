#version 450 core

layout(location = 0) in vec3 pos;
layout(location = 1) in vec2 tex_cords;
layout(location = 2) in vec3 normal;

// Color info:
layout(location = 3) in float use_texture;
layout(location = 4) in vec3 diffuse_color;
layout(location = 5) in vec3 ambient_color;
layout(location = 6) in vec3 specular_color;

// Instance info:
layout(location = 7) in vec4 model_mat1;
layout(location = 8) in vec4 model_mat2;
layout(location = 9) in vec4 model_mat3;
layout(location = 10) in vec4 model_mat4;

// Instanced normals info:
layout(location = 11) in vec3 n_matrix1;
layout(location = 12) in vec3 n_matrix2;
layout(location = 13) in vec3 n_matrix3;

layout(set = 0, binding = 0) uniform matrixUniform {
    mat4 proj_view_model_matrix;
    vec3 view_pos;
};

layout(location = 0) out vec2 frag_tex_cords;
layout(location = 1) out vec3 v_pos;
layout(location = 2) out vec3 v_normal;
// Color info:
layout(location = 3) out float f_use_texture;
layout(location = 4) out vec3 f_diffuse_color;
layout(location = 5) out vec3 f_ambient_color;
layout(location = 6) out vec3 f_specular_color;

void main(void) {
    mat4 model_matrix = mat4(model_mat1, model_mat2, model_mat3, model_mat4);
    mat3 n_matrix = mat3(n_matrix1, n_matrix2, n_matrix3);
    vec4 world_position = model_matrix * vec4(pos.xyz, 1.0);
    gl_Position = proj_view_model_matrix * world_position;
    frag_tex_cords = tex_cords;
    v_pos = world_position.xyz;
    v_normal = n_matrix * normal;
    f_use_texture = use_texture;
    f_diffuse_color = diffuse_color;
    f_ambient_color = ambient_color;
    f_specular_color = specular_color;
}