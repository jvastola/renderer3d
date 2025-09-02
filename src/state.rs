use winit::window::Window;
use winit::dpi::PhysicalSize;
use crate::camera::Camera;
use crate::renderer::Renderer;

pub struct State {
    pub size: PhysicalSize<u32>,
    pub camera: Camera,
    pub renderer: Renderer,
    // EntityManager can be added here if you want to manage entities in State
}

impl State {
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();
        let camera = Camera {
            eye: cgmath::Point3::new(1.5, 1.5, 2.0),
            target: cgmath::Point3::new(0.0, 0.0, 0.0),
            up: cgmath::Vector3::unit_y(),
            aspect: size.width as f32 / size.height as f32,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        };
        let renderer = Renderer::new(&camera);
        Self {
            size,
            camera,
            renderer,
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.camera.aspect = new_size.width as f32 / new_size.height as f32;
        }
    }
}
