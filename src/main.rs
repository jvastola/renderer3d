mod line_renderer;


mod camera;
mod mesh;
mod renderer;
mod state;
mod entity;
mod tesseract;
mod gfx_api;
mod software_rasterizer;
use cgmath::{Matrix4, Vector3, Point3, perspective, Rad};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use pixels::{Pixels, SurfaceTexture};
use crate::software_rasterizer::{SoftwareDevice, rgba};
use crate::mesh::{CUBE_VERTICES, CUBE_INDICES};
use crate::tesseract::Tesseract;
use std::time::Instant;

fn main() {
    let width = 800;
    let height = 600;
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Software Rasterizer 3D Demo")
        .with_inner_size(winit::dpi::LogicalSize::new(width as f64, height as f64))
        .build(&event_loop)
        .unwrap();
    let mut rasterizer = SoftwareDevice::new(width, height);

    // Cube mesh
    let cube_vertices: Vec<Vector3<f32>> = CUBE_VERTICES.iter().map(|v| Vector3::from(v.position)).collect();
    let cube_indices = CUBE_INDICES;

    // Tesseract
    let mut tesseract = Tesseract::new();

    let start_time = Instant::now();
    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let mut pixels = Pixels::new(width as u32, height as u32, surface_texture).unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => {}
            },
            Event::RedrawRequested(_) => {
                rasterizer.clear(rgba(30, 30, 30, 255));
                // Animate camera rotation
                let elapsed = start_time.elapsed().as_secs_f32();
                let radius = 2.0;
                let angle = elapsed * 0.5;
                let eye = Point3::new(radius * angle.cos(), 0.0, radius * angle.sin());
                let target = Point3::new(0.0, 0.0, 0.0);
                let up = Vector3::unit_y();
                let view = Matrix4::look_at_rh(eye, target, up);
                let proj = perspective(Rad(std::f32::consts::FRAC_PI_4), width as f32 / height as f32, 0.1, 10.0);
                let mvp = proj * view;

                // Draw cube mesh as triangles
                let mut screen_verts = vec![(0.0, 0.0, 0.0); cube_vertices.len()];
                for (i, v) in cube_vertices.iter().enumerate() {
                    let v4 = mvp * v.extend(1.0);
                    let ndc = v4.truncate() / v4.w;
                    let x = ((ndc.x + 1.0) * 0.5 * width as f32).clamp(0.0, width as f32 - 1.0);
                    let y = ((1.0 - ndc.y) * 0.5 * height as f32).clamp(0.0, height as f32 - 1.0);
                    let z = ndc.z;
                    screen_verts[i] = (x, y, z);
                }
                for tri in cube_indices.chunks(3) {
                    let v0 = screen_verts[tri[0] as usize];
                    let v1 = screen_verts[tri[1] as usize];
                    let v2 = screen_verts[tri[2] as usize];
                    let c0 = rgba(255, 0, 0, 255);
                    let c1 = rgba(0, 255, 0, 255);
                    let c2 = rgba(0, 0, 255, 255);
                    rasterizer.draw_triangle(v0, v1, v2, c0, c1, c2);
                }

                // Animate and draw tesseract as lines
                tesseract.rotate(0.01);
                let projected: Vec<Point3<f32>> = tesseract.vertices.iter().map(|v| {
                    // Project 4D to 3D
                    let factor = 2.0 / (2.0 - v.w);
                    Point3::new(v.x * factor, v.y * factor, v.z * factor)
                }).collect();
                let mut screen_tess = vec![(0.0, 0.0, 0.0); projected.len()];
                for (i, v) in projected.iter().enumerate() {
                    let v4 = mvp * v.to_homogeneous();
                    let ndc = v4.truncate() / v4.w;
                    let x = ((ndc.x + 1.0) * 0.5 * width as f32).clamp(0.0, width as f32 - 1.0);
                    let y = ((1.0 - ndc.y) * 0.5 * height as f32).clamp(0.0, height as f32 - 1.0);
                    let z = ndc.z;
                    screen_tess[i] = (x, y, z);
                }
                for &(i, j) in &tesseract.edges {
                    let a = screen_tess[i];
                    let b = screen_tess[j];
                    rasterizer.draw_line(a.0 as usize, a.1 as usize, b.0 as usize, b.1 as usize, rgba(255, 255, 0, 255));
                }

                // Copy framebuffer to pixels
                let frame = pixels.frame_mut();
                for (i, pixel) in rasterizer.framebuffer.iter().enumerate() {
                    let rgba = pixel.to_le_bytes();
                    let idx = i * 4;
                    frame[idx..idx+4].copy_from_slice(&rgba);
                }
                pixels.render().unwrap();
            }
            _ => {}
        }
        window.request_redraw();
    });
}
