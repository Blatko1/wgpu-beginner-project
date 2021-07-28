use crate::cube::{Cube, CubeType};
use crate::modeling::custom_models::quad;
use crate::modeling::vertex_index::Vertex;
use crate::quad::Quad;
use wgpu::util::DeviceExt;

pub struct Chunk {
    voxels: [Cube; 16 * 16 * 16],
    pub chunk_mesh: ChunkMesh,
}

const CHUNK_LENGTH: usize = 16;
const CHUNK_WIDTH: usize = 16;
const CHUNK_HEIGHT: usize = 16;

const CHUNK_SIZE: usize = CHUNK_WIDTH * CHUNK_LENGTH * CHUNK_HEIGHT;

impl Chunk {
    pub fn new(device: &wgpu::Device) -> Chunk {
        let default = Cube::default();
        let mut voxels: [Cube; CHUNK_SIZE] = [default; CHUNK_SIZE];

        let faces = Chunk::filter_unseen_quads(&mut voxels);

        let chunk_mesh = ChunkMesh::new(device, quad::VERTICES, quad::INDICES, faces);

        Chunk { voxels, chunk_mesh }
    }

    fn filter_unseen_quads(voxels: &mut [Cube; CHUNK_SIZE]) -> Vec<Quad> {
        let mut faces: Vec<Quad> = Vec::new();
        for y in 0..16 {
            for z in 0..16 {
                for x in 0..16 {
                    if voxels[x + 16 * z + 16 * 16 * y].is_active == false {
                        continue;
                    }
                    let mut left_face = true;
                    if x > 0 {
                        left_face = !voxels[(x - 1) + 16 * z + 16 * 16 * y].is_active;
                    }
                    let mut right_face = true;
                    if x < CHUNK_WIDTH - 1 {
                        right_face = !voxels[(x + 1) + 16 * z + 16 * 16 * y].is_active;
                    }
                    let mut back_face = true;
                    if z > 0 {
                        back_face = !voxels[x + 16 * (z - 1) + 16 * 16 * y].is_active;
                    }
                    let mut front_face = true;
                    if z < CHUNK_LENGTH - 1 {
                        front_face = !voxels[x + 16 * (z + 1) + 16 * 16 * y].is_active;
                    }
                    let mut bottom_face = true;
                    if y > 0 {
                        bottom_face = !voxels[x + 16 * z + 16 * 16 * (y - 1)].is_active;
                    }
                    let mut top_face = true;
                    if y < CHUNK_HEIGHT - 1 {
                        top_face = !voxels[x + 16 * z + 16 * 16 * (y + 1)].is_active;
                    }
                    voxels[x + 16 * z + 16 * 16 * y] = Cube::new(
                        top_face,
                        bottom_face,
                        left_face,
                        right_face,
                        back_face,
                        front_face,
                        CubeType::GRASS,
                    );
                    faces.append(
                        &mut voxels[x + 16 * z + 16 * 16 * y]
                            .get_faces([x as f32, y as f32, z as f32]),
                    );
                }
            }
        }
        return faces;
    }

    pub fn get_block(&self, x: usize, y: usize, z: usize) -> &Cube {
        &self.voxels[x + 16 * z + 16 * 16 * y]
    }
}

pub trait DrawChunk<'a> {
    fn draw_chunk(
        &mut self,
        chunk_mesh: &'a ChunkMesh,
        light_bind_group: &'a wgpu::BindGroup,
        matrix_bind_group: &'a wgpu::BindGroup,
        material_bind_group: &'a wgpu::BindGroup,
    );
}

impl<'a> DrawChunk<'a> for wgpu::RenderPass<'a> {
    fn draw_chunk(
        &mut self,
        chunk_mesh: &'a ChunkMesh,
        light_bind_group: &'a wgpu::BindGroup,
        matrix_bind_group: &'a wgpu::BindGroup,
        material_bind_group: &'a wgpu::BindGroup,
    ) {
        self.set_vertex_buffer(0, chunk_mesh.vertex_buffer.slice(..));
        self.set_vertex_buffer(1, chunk_mesh.instance_buffer.slice(..));
        self.set_bind_group(0, matrix_bind_group, &[]);
        self.set_bind_group(1, material_bind_group, &[]);
        self.set_bind_group(2, light_bind_group, &[]);
        self.set_index_buffer(chunk_mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);

        self.draw_indexed(
            0..chunk_mesh.indices_len as _,
            0,
            0..chunk_mesh.instances_len as _,
        );
    }
}

pub struct ChunkMesh {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    instance_buffer: wgpu::Buffer,
    indices_len: usize,
    instances_len: usize,
}

impl ChunkMesh {
    pub fn new(
        device: &wgpu::Device,
        vertices: &[Vertex],
        indices: &[u32],
        instances: Vec<Quad>,
    ) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsage::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsage::INDEX,
        });
        let instance_data = instances.iter().map(Quad::to_raw).collect::<Vec<_>>();
        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&instance_data),
            usage: wgpu::BufferUsage::VERTEX | wgpu::BufferUsage::COPY_DST,
        });
        let indices_len = indices.len();
        let instances_len = instances.len();
        Self {
            vertex_buffer,
            index_buffer,
            instance_buffer,
            indices_len,
            instances_len,
        }
    }
}
