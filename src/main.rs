use futures::executor::block_on;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod camera;
mod chunk;
mod cube;
mod debug_info;
mod generation;
mod light;
mod main_state;
mod mipmap;
mod modeling;
mod quad;
mod render_pipeline_tools;
mod texture;
mod uniform_matrix;

use main_state::State;
use winit::event::MouseButton;

fn main() {
    println!("Starting!");
    wgpu_subscriber::initialize_default_subscriber(None);
    //env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut state = block_on(State::new(&window));

    window.set_title("wgpu graphics");
    window.set_cursor_grab(true);
    window.set_cursor_visible(false);

    let mut mouse_input = true;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::DeviceEvent { event, .. } => {
                if mouse_input {
                    state.input(&event);
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
                    state.resize(new_size);
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    state.resize(*new_inner_size);
                }
                _ => (),
            },
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                state.update();

                match state.render() {
                    Ok(_) => {}
                    // Recreate the swap_chain if lost
                    Err(wgpu::SwapChainError::Lost) => state.resize(state.size),
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
