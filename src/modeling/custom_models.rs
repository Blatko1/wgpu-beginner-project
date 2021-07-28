pub mod quad {
    use crate::modeling::vertex_index::Vertex;
    pub const VERTICES: &[Vertex] = &[
        // tr //1
        Vertex {
            position: [-0.5, 0.5, -0.5],
            tex_cords: [1., 0.],
            normal: [0., 0., -1.],
        },
        // tl //2
        Vertex {
            position: [0.5, 0.5, -0.5],
            tex_cords: [0., 0.],
            normal: [0., 0., -1.],
        },
        // br //3
        Vertex {
            position: [-0.5, -0.5, -0.5],
            tex_cords: [1., 1.],
            normal: [0., 0., -1.],
        },
        // bl
        Vertex {
            position: [0.5, -0.5, -0.5],
            tex_cords: [0., 1.],
            normal: [0., 0., -1.],
        },
    ];

    #[rustfmt::skip]
    pub const INDICES: &[u32] = &[0, 1, 2,  1, 3, 2];
}

pub mod cube {
    use crate::modeling::vertex_index::Vertex;
    pub const _VERTICES: &[Vertex] = &[
        // tl
        Vertex {
            position: [0., 0., -1.],
            tex_cords: [0.5, 0.5],
            normal: [0., 0., 0.],
        },
        // tr
        Vertex {
            position: [1., 0., -1.],
            tex_cords: [0.5, 0.5],
            normal: [0., 0., 0.],
        },
        // bl
        Vertex {
            position: [0., -1., -1.],
            tex_cords: [0.5, 0.5],
            normal: [0., 0., 0.],
        },
        // br
        Vertex {
            position: [1., -1., -1.],
            tex_cords: [0.5, 0.5],
            normal: [0., 0., 0.],
        },
        // btl
        Vertex {
            position: [0., 0., 0.],
            tex_cords: [0.5, 0.5],
            normal: [0., 0., 0.],
        },
        // btr
        Vertex {
            position: [1., 0., 0.],
            tex_cords: [0.5, 0.5],
            normal: [0., 0., 0.],
        },
        // bbl
        Vertex {
            position: [0., -1., 0.],
            tex_cords: [0.5, 0.5],
            normal: [0., 0., 0.],
        },
        // bbr
        Vertex {
            position: [1., -1., 0.],
            tex_cords: [0.5, 0.5],
            normal: [0., 0., 0.],
        },
    ];

    #[rustfmt::skip]
    pub const _INDICES: &[u32] = &[
        0, 2, 1,  1, 2, 3,  2, 3, 5,  3, 7, 5,  5, 7, 4,  4, 7, 6,  4, 6, 2,  4, 2, 0
    ];
}
