#version 450 core

layout(location = 0) in vec3 fragColor;
layout(location = 1) in vec2 frag_tex_cords;

layout(location = 0) out vec4 outColor;

layout(set = 1, binding = 0) uniform texture2D t_diffuse;
layout(set = 1, binding = 1) uniform sampler s_diffuse;

void main(void) {
    outColor = texture(sampler2D(t_diffuse, s_diffuse), frag_tex_cords) /* vec4(fragColor.xyz, 1.0)*/;
}