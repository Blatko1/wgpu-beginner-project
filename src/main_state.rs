use crate::instance::{Instance, InstanceCollection, InstanceRaw};
use crate::model::{DrawModel, Mesh, Model};
use crate::texture::Texture;
use crate::uniform_matrix::MatrixUniform;
use crate::{
    camera::{Camera, CameraController},
    texture,
    vertex_index::{Vertex, VertexLayout},
};
use bytemuck::bytes_of;
use nalgebra::{Point3, Rotation3, Vector3};
use crate::generation::flat_terrain;

pub struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    pub size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    instance_collection: InstanceCollection,
    clear: wgpu::Color,
    camera: Camera,
    camera_controller: CameraController,
    matrix_uniform: MatrixUniform,
    depth_texture: Texture,
    rotation: f32,
}

const VERTICES: &[Vertex] = &[
    Vertex {
        //br
        position: [1.0, -1.0, 1.0],
        color: [0.5, 0.0, 0.0],
    },
    Vertex {
        //tl
        position: [-1.0, 1.0, 1.0],
        color: [0.0, 0.5, 0.0],
    },
    Vertex {
        //bl
        position: [-1.0, -1.0, 1.0],
        color: [0.0, 0.0, 0.5],
    },
    Vertex {
        //tr
        position: [1.0, 1.0, 1.0],
        color: [0.5, 0.0, 0.0],
    },
    Vertex {
        //br    4
        position: [1.0, -1.0, -1.0],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        //tr
        position: [1.0, 1.0, -1.0],
        color: [0.0, 0.0, 0.5],
    },
    Vertex {
        //bl
        position: [-1.0, -1.0, -1.0],
        color: [0.5, 0.0, 0.0],
    },
    Vertex {
        //tl
        position: [-1.0, 1.0, -1.0],
        color: [0.0, 0.5, 0.0],
    },
];

const INDICES: &[u16] = &[
    0, 1, 2, 0, 3, 1, 0, 5, 3, 0, 4, 5, 4, 6, 5, 5, 6, 7, 6, 2, 1, 6, 1, 7, 1, 3, 5, 5, 7, 1, 0, 2,
    4, 2, 6, 4,
];

impl State {
    pub async fn new(window: &winit::window::Window) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::NON_FILL_POLYGON_MODE,
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: adapter.get_swap_chain_preferred_format(&surface).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        let vert_shader =
            device.create_shader_module(&wgpu::include_spirv!("shaders/shader.vert.spv"));
        let frag_shader =
            device.create_shader_module(&wgpu::include_spirv!("shaders/shader.frag.spv"));

        let mut camera = Camera::new(
            Point3::new(0., 0., 2.),
            Point3::new(0., 0., 0.),
            &sc_desc,
            45.,
        );
        let camera_controller = CameraController::new();

        let mut matrix_uniform = MatrixUniform::new(&device, &camera);
        matrix_uniform.update_uniform(&mut camera);

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&matrix_uniform.bind_group_layout],
                push_constant_ranges: &[],
            });

        let depth_texture =
            texture::Texture::create_depth_texture(&device, &sc_desc, "depth_texture");

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vert_shader,
                entry_point: "main",
                buffers: &[
                    Vertex::init_buffer_layout(),
                    InstanceRaw::init_buffer_layout(),
                ],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                clamp_depth: false,
                polygon_mode: wgpu::PolygonMode::Line,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: texture::Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(wgpu::FragmentState {
                module: &frag_shader,
                entry_point: "main",
                targets: &[wgpu::ColorTargetState {
                    format: sc_desc.format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrite::ALL,
                }],
            }),
        });
        let clear = wgpu::Color {
            r: 0.1,
            g: 0.2,
            b: 0.3,
            a: 1.0,
        };
        let mesh = vec![Mesh::custom_mesh("Cube", &device, VERTICES, INDICES)];
        let terrain_mesh = vec![flat_terrain("flat", 10, 10, &device)];
        let terrain = Model { mesh: terrain_mesh };
        let model = Model { mesh };
        let instances = vec![
            Instance::new(
                Vector3::new(0., 0., 0.),
                Vector3::new(0., 0., 0.),
                Vector3::new(0., 0., 0.),
            ),
            /*Instance::new(
                Vector3::new(0., 0., 3.),
                Vector3::new(0., 0., 1.),
                Vector3::new(0., 0., 0.),
            ),
            Instance::new(
                Vector3::new(0., 0., 6.),
                Vector3::new(0., 0., 1.),
                Vector3::new(0., 0., 0.),
            ),
            Instance::new(
                Vector3::new(0., 0., 9.),
                Vector3::new(0., 0., 1.),
                Vector3::new(0., 0., 0.),
            ),*/
        ];
        let instance_collection =
            InstanceCollection::new("Model Instance Buffer", terrain, instances, &device);
        let rotation: f32 = 0.;

        State {
            surface,
            device,
            queue,
            sc_desc,
            swap_chain,
            size,
            render_pipeline,
            instance_collection,
            clear,
            camera,
            camera_controller,
            matrix_uniform,
            depth_texture,
            rotation,
        }
    }

    pub fn update(&mut self) {
        self.camera_controller.update_camera(&mut self.camera);
        self.matrix_uniform.update_uniform(&mut self.camera);
        self.queue.write_buffer(
            &self.matrix_uniform.buffer,
            0,
            bytemuck::cast_slice(&[self.matrix_uniform.proj_view_model_matrix]),
        );
        let instance_raw_vec = self
            .instance_collection
            .instances
            .iter()
            .map(Instance::to_raw)
            .collect::<Vec<_>>();
        self.queue.write_buffer(
            &self.instance_collection.buffer,
            0,
            bytemuck::cast_slice(&instance_raw_vec),
        );
        /*for i in &mut self.instance_collection.instances {
            i.rotate(Vector3::new(0.01, 0.01, 0.01));
        }*/
    }

    pub fn render(&self) -> Result<(), wgpu::SwapChainError> {
        let frame = self.swap_chain.get_current_frame()?.output;
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Command Encoder"),
            });
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &frame.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(self.clear),
                    store: true,
                },
            }],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.depth_texture.view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: true,
                }),
                stencil_ops: None,
            }),
        });

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.matrix_uniform.bind_group, &[]);
        render_pass.draw_mesh_instanced(
            &self.instance_collection.model,
            &self.instance_collection.buffer,
            0..self.instance_collection.instances.len() as _,
        );
        drop(render_pass);

        self.queue.submit(std::iter::once(encoder.finish()));

        Ok(())
    }

    pub fn input(&mut self, event: &winit::event::DeviceEvent) {
        self.camera_controller.process_input(event);
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
        self.camera.aspect = self.sc_desc.width as f32 / self.sc_desc.height as f32;
        self.depth_texture =
            texture::Texture::create_depth_texture(&self.device, &self.sc_desc, "depth_texture");
    }
}
