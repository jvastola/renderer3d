#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
}

impl Vertex {
    // Removed wgpu vertex attributes and layout for software rasterizer
}

pub const CUBE_VERTICES: &[Vertex] = &[
    Vertex { position: [-0.5, -0.5,  0.5] },
    Vertex { position: [ 0.5, -0.5,  0.5] },
    Vertex { position: [ 0.5,  0.5,  0.5] },
    Vertex { position: [-0.5,  0.5,  0.5] },
    Vertex { position: [-0.5, -0.5, -0.5] },
    Vertex { position: [ 0.5, -0.5, -0.5] },
    Vertex { position: [ 0.5,  0.5, -0.5] },
    Vertex { position: [-0.5,  0.5, -0.5] },
];

pub const CUBE_INDICES: &[u16] = &[
    0, 1, 2, 2, 3, 0,
    1, 5, 6, 6, 2, 1,
    5, 4, 7, 7, 6, 5,
    4, 0, 3, 3, 7, 4,
    3, 2, 6, 6, 7, 3,
    4, 5, 1, 1, 0, 4,
];
