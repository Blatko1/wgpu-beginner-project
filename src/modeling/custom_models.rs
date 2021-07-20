pub mod cube {
    use crate::vertex_index::Vertex;
    pub const VERTICES: &[Vertex] = &[
        Vertex {
            //br
            position: [1.0, -1.0, 1.0],
            tex_cords: [1.0, 0.0],
            color: [0.5, 0.0, 0.0],
        },
        Vertex {
            //tl
            position: [-1.0, 1.0, 1.0],
            tex_cords: [1.0, 0.0],
            color: [0.0, 0.5, 0.0],
        },
        Vertex {
            //bl
            position: [-1.0, -1.0, 1.0],
            tex_cords: [1.0, 0.0],
            color: [0.0, 0.0, 0.5],
        },
        Vertex {
            //tr
            position: [1.0, 1.0, 1.0],
            tex_cords: [1.0, 0.0],
            color: [0.5, 0.0, 0.0],
        },
        Vertex {
            //br    4
            position: [1.0, -1.0, -1.0],
            tex_cords: [1.0, 0.0],
            color: [0.0, 1.0, 0.0],
        },
        Vertex {
            //tr
            position: [1.0, 1.0, -1.0],
            tex_cords: [1.0, 0.0],
            color: [0.0, 0.0, 0.5],
        },
        Vertex {
            //bl
            position: [-1.0, -1.0, -1.0],
            tex_cords: [1.0, 0.0],
            color: [0.5, 0.0, 0.0],
        },
        Vertex {
            //tl
            position: [-1.0, 1.0, -1.0],
            tex_cords: [1.0, 0.0],
            color: [0.0, 0.5, 0.0],
        },
    ];

    pub const INDICES: &[u16] = &[
        0, 1, 2, 0, 3, 1, 0, 5, 3, 0, 4, 5, 4, 6, 5, 5, 6, 7, 6, 2, 1, 6, 1, 7, 1, 3, 5, 5, 7, 1, 0, 2,
        4, 2, 6, 4,
    ];
}