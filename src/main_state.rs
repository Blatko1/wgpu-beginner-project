use crate::chunk::{Chunk, DrawChunk};
use crate::debug_info::{DebugInfo, DebugInfoBuilder};
use crate::generation::flat_terrain;
use crate::light::Light;
use crate::modeling::instance::{Instance, InstanceRaw, ModelRenderInfo};
use crate::modeling::model::{DrawLight, DrawModel, Material, Model};
use crate::quad::QuadRaw;
use crate::render_pipeline_tools::new_render_pipeline;
use crate::texture::Texture;
use crate::uniform_matrix::MatrixUniform;
use crate::{
    camera::{Camera, CameraController},
    modeling::vertex_index::{Vertex, VertexLayout},
    texture,
};
use nalgebra::{Point3, Vector3};

pub struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    staging_belt: wgpu::util::StagingBelt,
    local_pool: futures::executor::LocalPool,
    local_spawner: futures::executor::LocalSpawner,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    pub size: winit::dpi::PhysicalSize<u32>,
    main_render_pipeline: wgpu::RenderPipeline,
    light_render_pipeline: wgpu::RenderPipeline,
    //model_info: ModelRenderInfo,
    light_info: ModelRenderInfo,
    clear: wgpu::Color,
    camera: Camera,
    camera_controller: CameraController,
    matrix_uniform: MatrixUniform,
    depth_texture: Texture,
    light_bind_group: wgpu::BindGroup,
    debug_info: DebugInfo,
    chunk: Chunk,
    chunk_texture: Material,
}

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

        let staging_belt = wgpu::util::StagingBelt::new(1024);
        let local_pool = futures::executor::LocalPool::new();
        let local_spawner = local_pool.spawner();

        let sc_format = adapter.get_swap_chain_preferred_format(&surface).unwrap();

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: sc_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);
        let mut camera = Camera::new(
            Point3::new(0., 0., 3.),
            Point3::new(0., 0., 1.),
            &sc_desc,
            45.,
        );
        let camera_controller = CameraController::new();

        let mut matrix_uniform = MatrixUniform::new(&device, &camera);
        matrix_uniform.update_uniform(&mut camera);

        let texture_layout = Texture::texture_bind_group_layout(&device);
        let light_layout = Light::bind_group_layout(&device);

        let depth_texture =
            texture::Texture::create_depth_texture(&device, &sc_desc, "depth_texture");

        // Main Render Pipeline
        let main_layouts = &[
            &matrix_uniform.bind_group_layout,
            &texture_layout,
            &light_layout,
        ];
        let main_render_pipeline = {
            let vert_shader =
                device.create_shader_module(&wgpu::include_spirv!("shaders/shader.vert.spv"));
            let frag_shader =
                device.create_shader_module(&wgpu::include_spirv!("shaders/shader.frag.spv"));
            new_render_pipeline(
                "main",
                &device,
                main_layouts,
                &vert_shader,
                &frag_shader,
                sc_desc.format,
                texture::Texture::DEPTH_FORMAT,
                &[Vertex::init_buffer_layout(), QuadRaw::init_buffer_layout()],
            )
        };

        // Lightning Pipeline
        let light = Light {
            position: [-10., 27., -8.],
            _padding: 0,
            color: [1., 1., 1.],
        };
        let light_bind_group = Light::new_light_buffer(light, &device, &light_layout);

        let light_render_pipeline = {
            let vert_light_shader =
                device.create_shader_module(&wgpu::include_spirv!("shaders/light.vert.spv"));
            let frag_light_shader =
                device.create_shader_module(&wgpu::include_spirv!("shaders/light.frag.spv"));
            new_render_pipeline(
                "light",
                &device,
                &[&matrix_uniform.bind_group_layout, &light_layout],
                &vert_light_shader,
                &frag_light_shader,
                sc_desc.format,
                texture::Texture::DEPTH_FORMAT,
                &[
                    Vertex::init_buffer_layout(),
                    InstanceRaw::init_buffer_layout(),
                ],
            )
        };

        let clear = wgpu::Color {
            r: 0.1,
            g: 0.4,
            b: 0.5,
            a: 1.0,
        };
        let res_dir = std::path::Path::new(env!("OUT_DIR")).join("res");

        // Custom model
        /*let car = Model::load(
            &device,
            &queue,
            &texture_layout,
            res_dir.join("car/mustang/mustang.obj"),
        )
        .unwrap();

        let instances = vec![Instance::new(
            Vector3::new(0., 0., 0.),
            Vector3::new(0., 0., 0.),
            Vector3::new(0., 0., 0.),
        )];
        let model_info = ModelRenderInfo::new("Model Instance Buffer", car, instances, &device);*/

        // Light object
        let light =
            Model::load(&device, &queue, &texture_layout, res_dir.join("test.obj")).unwrap();

        let light_instances = vec![Instance::new(
            Vector3::new(0., 0., 0.),
            Vector3::new(0., 0., 0.),
            Vector3::new(0., 0., 0.),
        )];
        let light_info =
            ModelRenderInfo::new("Model Instance Buffer", light, light_instances, &device);

        let debug_info = DebugInfoBuilder::new(10., 10., 20., sc_format, (size.width, size.height))
            .build(&device)
            .unwrap();
        let chunk = Chunk::new(&device);
        let chunk_texture = Material::custom_material(res_dir.join("trava.png"), &device, &queue);
        State {
            surface,
            device,
            queue,
            staging_belt,
            local_pool,
            local_spawner,
            sc_desc,
            swap_chain,
            size,
            main_render_pipeline,
            light_render_pipeline,
            //model_info,
            light_info,
            clear,
            camera,
            camera_controller,
            matrix_uniform,
            depth_texture,
            light_bind_group,
            debug_info,
            chunk,
            chunk_texture,
        }
    }

    pub fn update(&mut self) {
        self.camera_controller.update_camera(&mut self.camera);
        self.matrix_uniform.update_uniform(&mut self.camera);
        self.queue.write_buffer(
            &self.matrix_uniform.buffer,
            0,
            bytemuck::cast_slice(&[self.matrix_uniform.data]),
        );
        /*let instance_raw_vec = self
            .model_info
            .instances
            .iter()
            .map(Instance::to_raw)
            .collect::<Vec<_>>();
        self.queue.write_buffer(
            &self.model_info.instance_buffer,
            0,
            bytemuck::cast_slice(&instance_raw_vec),
        );*/
        /*for i in &mut self.model_info.instances {
            i.rotate(Vector3::new(0., 0., 0.005));
        }*/
    }

    pub fn render(&mut self) -> Result<(), wgpu::SwapChainError> {
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

        render_pass.set_pipeline(&self.light_render_pipeline);
        render_pass.set_bind_group(0, &self.matrix_uniform.bind_group, &[]);
        render_pass.draw_light(&self.light_info, &self.light_bind_group);

        render_pass.set_pipeline(&self.main_render_pipeline);
        /*render_pass.set_bind_group(0, &self.matrix_uniform.bind_group, &[]);
        render_pass.draw_model(&self.model_info, &self.light_bind_group);*/
        render_pass.draw_chunk(
            &self.chunk.chunk_mesh,
            &self.light_bind_group,
            &self.matrix_uniform.bind_group,
            &self.chunk_texture.bind_group,
        );

        drop(render_pass);

        self.debug_info
            .draw(
                &self.device,
                &mut self.staging_belt,
                &mut encoder,
                &frame.view,
                &self.camera,
            )
            .unwrap();

        self.staging_belt.finish();
        self.queue.submit(std::iter::once(encoder.finish()));

        // Recall unused staging buffers
        use futures::task::SpawnExt;

        self.local_spawner
            .spawn(self.staging_belt.recall())
            .expect("Recall staging belt");

        self.local_pool.run_until_stalled();

        unsafe {
            self.debug_info.update_info();
        }

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
        self.debug_info.resize(&self.size);
    }
}
