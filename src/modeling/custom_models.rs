pub mod cube {
    use crate::modeling::vertex_index::Vertex;
    pub const _VERTICES: &[Vertex] = &[
        Vertex {
            //br
            position: [1.0, -1.0, 1.0],
            tex_cords: [1.0, 0.0],
            normal: [0.0, 0.0, 0.0],
            use_texture: 0.,
            diffuse_color: [0.0, 0.0, 0.5],
            ambient_color: [0.1, 0.1, 0.1],
            specular_color: [0.1, 0.1, 0.1],
        },
        Vertex {
            //tl
            position: [-1.0, 1.0, 1.0],
            tex_cords: [0.0, 0.5],
            normal: [0.0, 0.0, 0.0],
            use_texture: 0.,
            diffuse_color: [0.0, 0.0, 0.5],
            ambient_color: [0.1, 0.1, 0.1],
            specular_color: [0.1, 0.1, 0.1],
        },
        Vertex {
            //bl
            position: [-1.0, -1.0, 1.0],
            tex_cords: [1.0, 0.0],
            normal: [1.0, 0.0, 0.0],
            use_texture: 0.,
            diffuse_color: [0.0, 0.0, 0.5],
            ambient_color: [0.1, 0.1, 0.1],
            specular_color: [0.1, 0.1, 0.1],
        },
        Vertex {
            //tr
            position: [1.0, 1.0, 1.0],
            tex_cords: [0.0, 0.0],
            normal: [0.0, 1.0, 0.0],
            use_texture: 0.,
            diffuse_color: [0.0, 0.0, 0.5],
            ambient_color: [0.1, 0.1, 0.1],
            specular_color: [0.1, 0.1, 0.1],
        },
        Vertex {
            //br    4
            position: [1.0, -1.0, -1.0],
            tex_cords: [1.0, 0.0],
            normal: [0.0, 0.0, 1.0],
            use_texture: 0.,
            diffuse_color: [0.0, 0.0, 0.5],
            ambient_color: [0.1, 0.1, 0.1],
            specular_color: [0.1, 0.1, 0.1],
        },
        Vertex {
            //tr
            position: [1.0, 1.0, -1.0],
            tex_cords: [0.0, 0.5],
            normal: [0.0, 0.0, 0.0],
            use_texture: 0.,
            diffuse_color: [0.0, 0.0, 0.5],
            ambient_color: [0.1, 0.1, 0.1],
            specular_color: [0.1, 0.1, 0.1],
        },
        Vertex {
            //bl
            position: [-1.0, -1.0, -1.0],
            tex_cords: [1.0, 0.0],
            normal: [0.0, 0.0, 0.0],
            use_texture: 0.,
            diffuse_color: [0.0, 0.0, 0.5],
            ambient_color: [0.1, 0.1, 0.1],
            specular_color: [0.1, 0.1, 0.1],
        },
        Vertex {
            //tl
            position: [-1.0, 1.0, -1.0],
            tex_cords: [1.0, 0.5],
            normal: [0.0, 0.0, 0.0],
            use_texture: 0.,
            diffuse_color: [0.0, 0.0, 0.5],
            ambient_color: [0.1, 0.1, 0.1],
            specular_color: [0.1, 0.1, 0.1],
        },
    ];

    pub const _INDICES: &[u32] = &[
        0, 1, 2, 0, 3, 1, 0, 5, 3, 0, 4, 5, 4, 6, 5, 5, 6, 7, 6, 2, 1, 6, 1, 7, 1, 3, 5, 5, 7, 1,
        0, 2, 4, 2, 6, 4,
    ];
}
