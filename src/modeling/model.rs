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
}

impl Material {
    #[allow(dead_code)]
    pub fn custom_material<P: AsRef<Path>>(
        path: /*Option<*/ P, /*>*/
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

        let mut diffuse_color: Vec<[f32; 3]> = Vec::new();
        let mut ambient_color: Vec<[f32; 3]> = Vec::new();
        let mut specular_color: Vec<[f32; 3]> = Vec::new();

        let mut use_texture: Vec<f32> = Vec::new();

        let mut materials = Vec::new();
        for mat in obj_materials {
            let diffuse_path = mat.normal_texture;
            println!("diff_map: {:?}", mat.diffuse_texture);
            println!("spec_map: {:?}", mat.specular_texture);
            println!("amb_map: {:?}", mat.ambient_texture);
            println!("Equals: {}", mat.diffuse_texture.eq(""));
            let diffuse_texture = if mat.diffuse_texture.eq("") {
                use_texture.push(0.);
                diffuse_color.push(mat.diffuse);
                Texture::load(device, queue, containing_folder.join("default.png"))?
            } else {
                use_texture.push(1.);
                diffuse_color.push([0., 0., 0.]);
                Texture::load(device, queue, containing_folder.join(mat.diffuse_texture))?
            };
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
                    tex_cords: { if use_texture[m.mesh.material_id.unwrap()] == 1. {
                        [m.mesh.texcoords[i * 2], m.mesh.texcoords[i * 2 + 1]]
                    } else {
                        [0., 0.]
                    }},
                    normal: [
                        m.mesh.normals[i * 3],
                        m.mesh.normals[i * 3 + 1],
                        m.mesh.normals[i * 3 + 2],
                    ],
                    // Color info:
                    use_texture: 0.,
                    diffuse_color: diffuse_color[m.mesh.material_id.unwrap()],
                    ambient_color: ambient_color[m.mesh.material_id.unwrap()],
                    specular_color: specular_color[m.mesh.material_id.unwrap()],
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
