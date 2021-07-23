use wgpu_glyph::{ab_glyph, GlyphBrushBuilder, Section, Text};
use crate::camera::{Camera};
use std::time::{SystemTime, UNIX_EPOCH, Duration};

pub struct DebugInfoBuilder {
    position: (f32, f32),
    scale: f32,
    render_format: wgpu::TextureFormat,
    screen_bounds: (u32, u32),
}

impl DebugInfoBuilder {
    pub fn new(
        x: f32,
        y: f32,
        scale: f32,
        render_format: wgpu::TextureFormat,
        screen_bounds: (u32, u32),
    ) -> Self {
        Self {
            position: (x, y),
            scale,
            render_format,
            screen_bounds,
        }
    }

    pub fn build(&self, device: &wgpu::Device) -> Result<DebugInfo, Box<dyn std::error::Error>> {
        let font =
            ab_glyph::FontArc::try_from_slice(include_bytes!("fonts/Inconsolata-Regular.ttf"))?;
        let brush = GlyphBrushBuilder::using_font(font).build(device, self.render_format);

        let info = DebugInfo {
            position: self.position,
            scale: self.scale,
            screen_bounds: self.screen_bounds,
            brush,
            text: vec![DebugTools::FPS, DebugTools::Position],
            fps: 0.
        };
        Ok(info)
    }
}

pub struct DebugInfo {
    position: (f32, f32),
    scale: f32,
    screen_bounds: (u32, u32),
    brush: wgpu_glyph::GlyphBrush<()>,
    text: Vec<DebugTools>,
    fps: f64
}

static mut TIME: Duration = Duration::ZERO;
static mut FPS_SHOW_TIME: Duration = Duration::ZERO;

impl DebugInfo {
    pub fn draw<'a>(
        &mut self,
        device: &wgpu::Device,
        staging_belt: &mut wgpu::util::StagingBelt,
        encoder: &mut wgpu::CommandEncoder,
        target: &wgpu::TextureView,
        camera: &Camera
    ) {
        let fps = String::from(format!("FPS: {:.2}\n", self.fps));
        let pos = String::from(format!("Position: x: {:.2}, y: {:.2}, z: {:.2}\n", camera.eye.x, camera.eye.y, camera.eye.z));
        let mut debug_text: Vec<Text> = Vec::new();
        for t in self.text.iter() {
            match t {
                DebugTools::FPS => {
                    debug_text.push(Text::new(&fps).with_color([1., 1., 1., 1.]).with_scale(self.scale));
                },
                DebugTools::Position => {
                    debug_text.push(Text::new(&pos).with_color([1., 1., 1., 1.]).with_scale(self.scale));
                }
            }
        }
        self.brush.queue(Section {
            screen_position: (self.position.0, self.position.1),
            bounds: (self.screen_bounds.0 as f32, self.screen_bounds.1 as f32),
            layout: Default::default(),
            text: debug_text,
        });
        self.brush.draw_queued(
            device,
            staging_belt,
            encoder,
            target,
            self.screen_bounds.0,
            self.screen_bounds.1,
        );
    }

    pub unsafe fn update_info(&mut self) {
        let now = SystemTime::now();
        let time = now.duration_since(UNIX_EPOCH).expect("Time went backwards!");
        if (time.as_millis() - FPS_SHOW_TIME.as_millis()) > 1000 {
            let delta = time.as_micros() - TIME.as_micros();
            self.fps = 1./(delta as f64/1000000.);
            FPS_SHOW_TIME = time;
        }
        TIME = time;
    }

    pub fn rearrange(&mut self, text: Vec<DebugTools>) {
        self.text = text;
    }
}

pub enum DebugTools {
    FPS,
    Position
}