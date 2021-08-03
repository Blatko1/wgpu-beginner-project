use winit::{
    event::{ElementState, Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{self, WindowBuilder},
};

pub struct ClientWindow {
    pub event_loop: EventLoop<()>,
    pub window: window::Window,
}

impl ClientWindow {}
