use crate::entity::Entity;
use crate::line_renderer::LineVertex;
use cgmath::{Point3, Vector4};

/// Projects a 4D point to 3D using perspective projection
fn project_4d_to_3d(p: Vector4<f32>, w: f32) -> Point3<f32> {
    let factor = w / (w - p.w);
    Point3::new(p.x * factor, p.y * factor, p.z * factor)
}

pub struct Tesseract {
    pub vertices: Vec<Vector4<f32>>,
    pub edges: Vec<(usize, usize)>,
    pub rotation: f32,
}

impl Tesseract {
    pub fn new() -> Self {
        // 16 vertices of a tesseract
        let mut vertices = Vec::new();
        for i in 0..16 {
            let x = if i & 1 == 0 { -0.5 } else { 0.5 };
            let y = if i & 2 == 0 { -0.5 } else { 0.5 };
            let z = if i & 4 == 0 { -0.5 } else { 0.5 };
            let w = if i & 8 == 0 { -0.5 } else { 0.5 };
            vertices.push(Vector4::new(x, y, z, w));
        }
        // Edges: connect vertices differing by one bit
        let mut edges = Vec::new();
        for i in 0..16 {
            for j in 0..4 {
                let k = i ^ (1 << j);
                if i < k {
                    edges.push((i, k));
                }
            }
        }
        Self { vertices, edges, rotation: 0.0 }
    }
    /// Rotates the tesseract in 4D space
    pub fn rotate(&mut self, angle: f32) {
        self.rotation += angle;
        let cos = angle.cos();
        let sin = angle.sin();
        for v in &mut self.vertices {
            // Simple rotation in zw plane
            let z = v.z * cos - v.w * sin;
            let w = v.z * sin + v.w * cos;
            v.z = z;
            v.w = w;
        }
    }
}

impl Entity for Tesseract {
    fn update(&mut self, dt: f32) {
        self.rotate(dt * 0.5);
    }
    fn line_vertices(&self) -> Vec<LineVertex> {
        // Project 4D vertices to 3D
        let projected: Vec<Point3<f32>> = self.vertices.iter().map(|v| project_4d_to_3d(*v, 2.0)).collect();
        let mut line_vertices = Vec::new();
        for &(i, j) in &self.edges {
            let a = projected[i];
            let b = projected[j];
            line_vertices.push(LineVertex { position: [a.x, a.y, a.z], color: [1.0, 1.0, 0.0] });
            line_vertices.push(LineVertex { position: [b.x, b.y, b.z], color: [1.0, 1.0, 0.0] });
        }
        line_vertices
    }
}
