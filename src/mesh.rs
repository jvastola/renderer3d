#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
}

impl Vertex {
    pub const ATTRIBS: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![0 => Float32x3];
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
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
