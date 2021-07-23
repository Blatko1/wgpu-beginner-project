use crate::modeling::vertex_index::Vertex;
use crate::texture::Texture;
use anyhow::Context;
use anyhow::Result;
use std::fs;
use std::ops::Range;
use std::path::Path;
use tobj::LoadOptions;
use wgpu::util::DeviceExt;

pub struct Material {
    pub texture: Texture,
    pub bind_group: wgpu::BindGroup,
}

impl Material {
    pub fn custom_material<P: AsRef<Path>>(
        path: P,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Self {
        let diffuse_bytes = fs::read(path.as_ref()).unwrap();
        let texture = Texture::from_bytes(
            &device,
            &queue,
            diffuse_bytes.as_slice(),
            path.as_ref().to_str().unwrap(),
        )
        .unwrap();
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &Texture::texture_bind_group_layout(&device),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                },
            ],
            label: Some(&format!(
                "{} diffuse_bind_group",
                path.as_ref().to_str().unwrap()
            )),
        });

        Self {
            texture,
            bind_group,
        }
    }
}

pub struct Mesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_length: u32,
    pub material: usize,
}

impl Mesh {
    #[allow(dead_code)]
    pub fn custom_mesh<P: AsRef<Path>>(
        name: P,
        device: &wgpu::Device,
        vertices: &[Vertex],
        indices: &[u32],
        material: usize,
    ) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{:?} Vertex Buffer", name.as_ref())),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsage::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{:?} Index Buffer", name.as_ref())),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsage::INDEX,
        });
        let index_length = indices.len() as u32;
        Self {
            vertex_buffer,
            index_buffer,
            index_length,
            material,
        }
    }
}

pub struct Model {
    pub mesh: Vec<Mesh>,
    pub material: Vec<Material>,
}

impl Model {
    pub fn load<P: AsRef<Path>>(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        layout: &wgpu::BindGroupLayout,
        path: P,
    ) -> Result<Self> {
        let (obj_models, obj_materials) = tobj::load_obj(
            path.as_ref(),
            &LoadOptions {
                triangulate: true,
                single_index: true,
                ..Default::default()
            },
        )?;

        let obj_materials = obj_materials?;

        // We're assuming that the texture files are stored with the obj file
        let containing_folder = path.as_ref().parent().context("Directory has no parent")?;

        let mut materials = Vec::new();
        for mat in obj_materials {
            let diffuse_path = mat.diffuse_texture;
            let diffuse_texture =
                Texture::load(device, queue, containing_folder.join(diffuse_path))?;

            let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                    },
                ],
                label: None,
            });

            materials.push(Material {
                texture: diffuse_texture,
                bind_group,
            });
        }

        let mut meshes = Vec::new();
        for m in obj_models {
            let mut vertices = Vec::new();
            for i in 0..m.mesh.positions.len() / 3 {
                vertices.push(Vertex {
                    position: [
                        m.mesh.positions[i * 3],
                        m.mesh.positions[i * 3 + 1],
                        m.mesh.positions[i * 3 + 2],
                    ],
                    tex_cords: [m.mesh.texcoords[i * 2], m.mesh.texcoords[i * 2 + 1]],
                    color: [0.0, 1.0, 0.0],
                    normal: [
                        m.mesh.normals[i * 3],
                        m.mesh.normals[i * 3 + 1],
                        m.mesh.normals[i * 3 + 2],
                    ],
                });
            }
            let tmp_mesh = Mesh::custom_mesh(
                path.as_ref().as_os_str().to_str().unwrap(),
                &device,
                vertices.as_slice(),
                m.mesh.indices.as_slice(),
                m.mesh.material_id.unwrap_or(0),
            );
            meshes.push(tmp_mesh);
        }

        Ok(Self {
            mesh: meshes,
            material: materials,
        })
    }
}

pub trait DrawModel<'a> {
    fn draw_mesh(&mut self, model: &'a Model, instance_buffer: &'a wgpu::Buffer);
    fn draw_mesh_instanced(
        &mut self,
        model: &'a Model,
        instance_buffer: &'a wgpu::Buffer,
        instances: Range<u32>,
    );
}

impl<'a> DrawModel<'a> for wgpu::RenderPass<'a> {
    fn draw_mesh(&mut self, model: &'a Model, instance_buffer: &'a wgpu::Buffer) {
        self.draw_mesh_instanced(model, instance_buffer, 0..1);
    }

    fn draw_mesh_instanced(
        &mut self,
        model: &'a Model,
        instance_buffer: &'a wgpu::Buffer,
        instances: Range<u32>,
    ) {
        for m in &model.mesh {
            self.set_vertex_buffer(0, m.vertex_buffer.slice(..));
            self.set_vertex_buffer(1, instance_buffer.slice(..));
            self.set_bind_group(1, &model.material.get(m.material).unwrap().bind_group, &[]);
            self.set_index_buffer(m.index_buffer.slice(..), wgpu::IndexFormat::Uint32);

            self.draw_indexed(0..m.index_length, 0, instances.clone());
        }
    }
}
