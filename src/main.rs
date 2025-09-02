
mod camera;
mod mesh;
mod renderer;
mod state;

use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;
use state::State;

fn main() {
    use std::time::Instant;

    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).unwrap();
    let mut state = pollster::block_on(State::new(&window));
    let start_time = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::Resized(physical_size) => {
                    state.resize(physical_size);
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    state.resize(*new_inner_size);
                }
                _ => {}
            },
            Event::RedrawRequested(_) => {
                // Automatically rotate camera around the scene
                let elapsed = start_time.elapsed().as_secs_f32();
                let radius = 2.0;
                let angle = elapsed * 0.5; // radians per second
                state.camera.eye.x = radius * angle.cos();
                state.camera.eye.z = radius * angle.sin();
                state.camera.eye.y = 1.5;
                state.camera.target = cgmath::Point3::new(0.0, 0.0, 0.0);
                state.renderer.update_camera(&state.queue, &state.camera);

                match state.renderer.render(&state.device, &state.queue, &state.surface, &state.config) {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(_) => {}
                }
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        }
    });
}
