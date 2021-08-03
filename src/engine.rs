use crate::camera::Camera;
use crate::graphics::Graphics;
use crate::renderer::Renderer;
use crate::rendering::renderer::Renderer;
use crate::rendering::graphics::Graphics;
use crate::world::World;
use crate::chunk::Chunk;
use crate::rendering::object::Object;

pub struct Engine {
    renderer: Renderer,
    world: World
}

impl Engine {
    pub fn new(graphics: &Graphics) -> Self {
        let renderer = Renderer::new(graphics);

        Self { renderer }
    }

    pub fn render(&self, graphics: &Graphics) -> Result<(), wgpu::SwapChainError> {
        self.renderer.render(&graphics)?;
        Ok(())
    }

    pub fn update(&self) {}

    pub fn input(&self) {}
}
