use nalgebra::{Matrix4, Point3, Rotation3, Translation3, Unit, Vector3};
use winit::event::{DeviceEvent, ElementState, KeyboardInput, VirtualKeyCode};

#[rustfmt::skip]
const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

pub struct Camera {
    pub eye: Point3<f32>,
    target: Point3<f32>,
    up: Vector3<f32>,
    pub aspect: f32,
    fov: f32,
    near: f32,
    far: f32,
    rotation_axis: Vector3<f32>,
    angle: f32,
    radius: Vector3<f32>,
    translation: Vector3<f32>,
    camera_controller: CameraController,
}

impl Camera {
    pub fn new(
        eye: Point3<f32>,
        target: Point3<f32>,
        sc_desc: &wgpu::SwapChainDescriptor,
        fov: f32,
    ) -> Self {
        Self {
            eye,
            target,
            up: Vector3::y(),
            aspect: sc_desc.width as f32 / sc_desc.height as f32,
            fov,
            near: 0.1,
            far: 100.0,
            rotation_axis: Vector3::new(0., 0., 0.),
            angle: 0.0,
            radius: Vector3::new(0., 0., 0.),
            translation: Vector3::new(0., 0., 0.),
            camera_controller: CameraController::new(),
        }
    }

    pub fn create_view_proj_model_matrix(&self) -> Matrix4<f32> {
        let target = Point3::new(
            self.eye.x + self.target.x,
            self.eye.y + self.target.y,
            self.eye.z + self.target.z,
        );

        let translation =
            Translation3::new(self.translation.x, self.translation.y, self.translation.z)
                .to_homogeneous();
        let rot = Rotation3::from_axis_angle(&Unit::new_normalize(self.rotation_axis), self.angle)
            .matrix()
            .to_homogeneous();
        let radius_from_center =
            Translation3::new(self.radius.x, self.radius.y, self.radius.z).to_homogeneous();
        let view = Matrix4::look_at_rh(&self.eye, &target, &self.up);
        let model = radius_from_center * rot * translation;
        let proj = Matrix4::new_perspective(self.aspect, self.fov, self.near, self.far);
        let result = OPENGL_TO_WGPU_MATRIX * proj * view * model.try_inverse().unwrap();
        return result;
    }
}

pub struct CameraController {
    speed: f32,
    forward: f32,
    backward: f32,
    left: f32,
    right: f32,
    up: f32,
    down: f32,
    yaw: f32,
    pitch: f32,
}

impl CameraController {
    pub fn new() -> Self {
        CameraController {
            speed: 0.03,
            forward: 0.,
            backward: 0.,
            left: 0.,
            right: 0.,
            up: 0.,
            down: 0.,
            yaw: 270.0,
            pitch: 0.0,
        }
    }

    pub fn process_input(&mut self, event: &DeviceEvent) -> bool {
        match event {
            DeviceEvent::MouseMotion { delta } => {
                self.yaw += (delta.0 * 0.1) as f32;
                self.pitch -= (delta.1 * 0.1) as f32;

                if self.pitch > 89.0 {
                    self.pitch = 89.0;
                } else if self.pitch < -89.0 {
                    self.pitch = -89.0;
                }
                true
            }
            DeviceEvent::Key(KeyboardInput {
                state,
                virtual_keycode,
                ..
            }) => {
                let value: f32;
                if *state == ElementState::Pressed {
                    value = 1.0;
                } else {
                    value = 0.;
                }
                match virtual_keycode.unwrap() {
                    VirtualKeyCode::Space => {
                        self.up = value;
                        true
                    }
                    VirtualKeyCode::LShift => {
                        self.down = value;
                        true
                    }
                    VirtualKeyCode::W => {
                        self.forward = value;
                        true
                    }
                    VirtualKeyCode::S => {
                        self.backward = value;
                        true
                    }
                    VirtualKeyCode::A => {
                        self.left = value;
                        true
                    }
                    VirtualKeyCode::D => {
                        self.right = value;
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }

    pub fn update_camera(&mut self, camera: &mut Camera) {
        if self.yaw > 360.0 {
            self.yaw = 0.0;
        } else if self.yaw < 0.0 {
            self.yaw = 360.0;
        }
        camera.target = Point3::new(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        );
        let target = Vector3::new(camera.target.x, 0.0, camera.target.z).normalize();
        camera.eye += &target * self.speed * (self.forward - self.backward);
        camera.eye += &target.cross(&camera.up) * self.speed * (self.right - self.left);
        camera.eye += Vector3::new(0.0, 1.0, 0.0) * self.speed * (self.up - self.down);
    }
}
