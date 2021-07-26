use crate::modeling::instance::ModelRenderInfo;
use crate::modeling::vertex_index::Vertex;
use crate::texture::Texture;
use anyhow::Context;
use anyhow::Result;
use std::fs;
use std::path::Path;
use tobj::LoadOptions;
use wgpu::util::DeviceExt;

pub struct Material {
    pub texture: Texture,
    pub bind_group: wgpu::BindGroup,
    //pub colors: Vec<Color>,
    //pub color_buffer: wgpu::Buffer
}

impl Material {
    #[allow(dead_code)]
    pub fn custom_material<P: AsRef<Path>>(
        path: /*Option<*/P/*>*/,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        //diffuse_color: Option<Vec<[f32; 3]>>
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
        /*let colors = Color {
            ambient_color: [1., 1., 1.],
            diffuse_color: match diffuse_color {
                None => [0., 0., 0.],
                Some(d) => d
            },
            specular_color: [0., 0., 0.],
            shininess: 0.0,
            include_texture: 1.,
            include_diff_color: 1.
        };

        let color_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!(
                "{} color:buffer",
                path.as_ref().to_str().unwrap()
            )),
            contents: bytemuck::cast_slice(&[colors]),
            usage: wgpu::BufferUsage::VERTEX
        })*/

        Self {
            texture,
            bind_group,
            //colors,
            //color_buffer
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

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Color {
    ambient_color: [f32; 3],
    diffuse_color: [f32; 3],
    specular_color: [f32; 3],
    shininess: f32,
    include_texture: f32,
    include_diff_color: f32
}

impl Color {
    pub fn init_buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 11,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 12,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: std::mem::size_of::<[f32; 6]>() as wgpu::BufferAddress,
                    shader_location: 13,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32,
                    offset: std::mem::size_of::<[f32; 9]>() as wgpu::BufferAddress,
                    shader_location: 14,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32,
                    offset: std::mem::size_of::<[f32; 10]>() as wgpu::BufferAddress,
                    shader_location: 15,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32,
                    offset: std::mem::size_of::<[f32; 11]>() as wgpu::BufferAddress,
                    shader_location: 16,
                },
            ],
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

        let mut diffuse_color: Vec<[f32; 3]>  = Vec::new();
        let mut ambient_color: Vec<[f32; 3]>  = Vec::new();
        let mut specular_color: Vec<[f32; 3]>  = Vec::new();

        let mut materials = Vec::new();
        for mat in obj_materials {
            let diffuse_path = mat.normal_texture;
            let diffuse_texture =
                Texture::load(device, queue, containing_folder.join("happy.png"))?;
            diffuse_color.push(mat.diffuse);
            ambient_color.push(mat.ambient);
            specular_color.push(mat.specular);
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
                    tex_cords: [0., 0.],
                    color: diffuse_color[m.mesh.material_id.unwrap()],
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
    fn draw_model(&mut self, model_info: &'a ModelRenderInfo, light: &'a wgpu::BindGroup);
}

impl<'a> DrawModel<'a> for wgpu::RenderPass<'a> {
    fn draw_model(
        &mut self,
        model_info: &'a ModelRenderInfo,
        light_bind_group: &'a wgpu::BindGroup,
    ) {
        self.set_vertex_buffer(1, model_info.instance_buffer.slice(..));
        self.set_bind_group(2, light_bind_group, &[]);
        for m in &model_info.model.mesh {
            self.set_vertex_buffer(0, m.vertex_buffer.slice(..));
            self.set_bind_group(
                1,
                &model_info
                    .model
                    .material
                    .get(m.material)
                    .unwrap()
                    .bind_group,
                &[],
            );
            self.set_index_buffer(m.index_buffer.slice(..), wgpu::IndexFormat::Uint32);

            self.draw_indexed(0..m.index_length, 0, 0..model_info.instances.len() as _);
        }
    }
}

pub trait DrawLight<'a> {
    fn draw_light(&mut self, light_info: &'a ModelRenderInfo, light: &'a wgpu::BindGroup);
}

impl<'a> DrawLight<'a> for wgpu::RenderPass<'a> {
    fn draw_light(
        &mut self,
        light_info: &'a ModelRenderInfo,
        light_bind_group: &'a wgpu::BindGroup,
    ) {
        self.set_vertex_buffer(1, light_info.instance_buffer.slice(..));
        self.set_bind_group(1, light_bind_group, &[]);
        for m in &light_info.model.mesh {
            self.set_vertex_buffer(0, m.vertex_buffer.slice(..));
            self.set_index_buffer(m.index_buffer.slice(..), wgpu::IndexFormat::Uint32);

            self.draw_indexed(0..m.index_length, 0, 0..light_info.instances.len() as _);
        }
    }
}
