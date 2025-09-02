use crate::camera::Camera;
use crate::mesh::{Vertex, CUBE_VERTICES, CUBE_INDICES};
use crate::line_renderer::{LineRenderer, LineVertex};
use crate::gfx_api::{Device, Buffer, Shader, Pipeline, CommandQueue, DummyDevice, DummyCommandQueue, DummyShader, DummyPipeline, DummyBuffer, GfxResult};

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub view_proj: [[f32; 4]; 4],
}

pub struct Renderer {
    pub camera_uniform: CameraUniform,
    pub num_indices: u32,
    pub line_renderer: LineRenderer,
    pub device: DummyDevice,
    pub vertex_buffer: Box<dyn Buffer>,
    pub index_buffer: Box<dyn Buffer>,
    pub shader: Box<dyn Shader>,
    pub pipeline: Box<dyn Pipeline>,
    pub command_queue: DummyCommandQueue,
}

impl Renderer {
    pub fn new(camera: &Camera) -> Self {
        let device = DummyDevice;
        let command_queue = DummyCommandQueue::new();
        let camera_uniform = CameraUniform {
            view_proj: camera.build_view_projection_matrix().into(),
        };
        let vertex_buffer = device.create_buffer(CUBE_VERTICES.len() * std::mem::size_of::<Vertex>()).unwrap();
        let index_buffer = device.create_buffer(CUBE_INDICES.len() * std::mem::size_of::<u16>()).unwrap();
        let shader = device.create_shader("// vertex and fragment shader source").unwrap();
        let pipeline = device.create_pipeline(&*shader).unwrap();
        let line_renderer = LineRenderer::new(device.create_pipeline(&*shader).unwrap());
        let num_indices = CUBE_INDICES.len() as u32;
        Self {
            camera_uniform,
            num_indices,
            line_renderer,
            device,
            vertex_buffer,
            index_buffer,
            shader,
            pipeline,
            command_queue,
        }
    }

    pub fn update_camera(&mut self, camera: &Camera) {
        self.camera_uniform.view_proj = camera.build_view_projection_matrix().into();
        // Example: update buffer with new camera data
        let _ = self.vertex_buffer.write(bytemuck::cast_slice(CUBE_VERTICES));
    }

    pub fn render_with_lines(&mut self, line_vertices: &[LineVertex]) -> GfxResult<()> {
        self.pipeline.bind()?;
        self.command_queue.add_command("draw cube")?;
        if !line_vertices.is_empty() {
            self.command_queue.add_command("draw lines")?;
        }
        self.device.submit(&self.command_queue)?;
        self.command_queue.clear();
        Ok(())
    }
}
