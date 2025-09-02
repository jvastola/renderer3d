
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LineVertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl LineVertex {
    // Removed wgpu vertex attributes and layout for software rasterizer
}

use crate::gfx_api::Pipeline;

pub struct LineRenderer {
    pub pipeline: Box<dyn Pipeline>,
}

impl LineRenderer {
    pub fn new(pipeline: Box<dyn Pipeline>) -> Self {
        Self { pipeline }
    }
}
