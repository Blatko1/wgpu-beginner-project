#version 450 core

#extension GL_EXT_nonuniform_qualifier : require

layout(location = 0) in vec2 frag_tex_cords;
layout(location = 1) in vec3 v_pos;
layout(location = 2) in vec3 v_normal;
layout(location = 3) flat in uint v_offset;

layout(location = 0) out vec4 outColor;

layout(set = 0, binding = 0) uniform matrixUniform {
    mat4 proj_view_model_matrix;
    vec3 view_pos;
};
layout(set = 1, binding = 0) uniform sampler u_sampler;
layout(set = 1, binding = 1) uniform texture2D u_textures[];
layout(set = 2, binding = 0) uniform Light {
    vec3 light_pos;
    vec3 light_color;
};
float ambient_strenght = 0.05;

void main(void) {
    vec4 object_texture = texture(sampler2D(u_textures[v_offset], u_sampler), frag_tex_cords);

    vec3 ambient_color = light_color * ambient_strenght;

    vec3 normal = normalize(v_normal);
    vec3 light_dir = normalize(light_pos - v_pos);

    vec3 view_dir = normalize(view_pos - v_pos);
    vec3 half_dir = normalize(view_dir + light_dir);

    float specular_strength = pow(max(dot(half_dir, normal), 0.0), 500.0);
    vec3 specular_color = specular_strength * light_color;

    float diffuse_strenght = max(dot(light_dir, normal), 0.0);
    vec3 diffuse_color = light_color * diffuse_strenght;

    vec3 result = (ambient_color + diffuse_color + specular_color) * object_texture.xyz;
    outColor = vec4(result, 0.3);
}