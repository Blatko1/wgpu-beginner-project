mod camera;
mod chunk;
mod cube;
mod debug_info;
mod engine;
mod generation;
mod light;
mod main_state;
mod mipmap;
mod modeling;
mod quad;
mod render_pipeline_tools;
mod texture;
mod uniform_matrix;
mod window;
mod rendering;
mod world;

use winit::event::{ElementState, Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

use crate::engine::Engine;
use crate::graphics::Graphics;
use crate::window::ClientWindow;
use crate::rendering::graphics::Graphics;

pub struct Client {
    graphics: Graphics,
    engine: Engine,
}

impl Client {
    pub fn new(window: &Window) -> Self {
        let graphics = Graphics::new(window).expect("Failed to create graphics!");

        let engine = Engine::new(&graphics).expect("Failed to create an engine.");

        Self { graphics, engine }
    }

    pub fn render(&self) -> Result<(), wgpu::SwapChainError> {
        self.engine.render(&self.graphics)?;
        Ok(())
    }

    pub fn update() {}
}

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    window.set_title("The Voxel Engine");
    window.set_cursor_grab(true);
    window.set_cursor_visible(false);

    let client = Client::new(&window);

    let mut mouse_input = true;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::DeviceEvent { event, .. } => {
                if mouse_input {
                    client.engine.input(&event);
                }
            }
            Event::WindowEvent { window_id, event } if window_id == window.id() => match event {
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => *control_flow = ControlFlow::Exit,
                WindowEvent::MouseInput { button, .. } => match button {
                    MouseButton::Left => {
                        window.set_cursor_grab(true);
                        window.set_cursor_visible(false);
                        mouse_input = true;
                    }
                    MouseButton::Right => {
                        window.set_cursor_grab(false);
                        window.set_cursor_visible(true);
                        mouse_input = false;
                    }
                    _ => {}
                },
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(new_size) => {
                    client.resize(new_size);
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    client.resize(*new_inner_size);
                }
                _ => (),
            },
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                client.update();

                match state.render() {
                    Ok(_) => {}
                    // Recreate the swap_chain if lost
                    Err(wgpu::SwapChainError::Lost) => client.resize(client.size),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            _ => (),
        }
    })
}
