// Minimal software rasterizer API and implementation

pub struct SoftwareDevice {
    pub framebuffer: Vec<u32>, // RGBA8 pixels
    pub depthbuffer: Vec<f32>, // Depth values
    pub width: usize,
    pub height: usize,
}

impl SoftwareDevice {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            framebuffer: vec![0; width * height],
            depthbuffer: vec![f32::INFINITY; width * height],
            width,
            height,
        }
    }

    pub fn clear(&mut self, color: u32) {
        for pixel in self.framebuffer.iter_mut() {
            *pixel = color;
        }
        for depth in self.depthbuffer.iter_mut() {
            *depth = f32::INFINITY;
        }
    }

    pub fn draw_pixel(&mut self, x: usize, y: usize, color: u32, depth: f32) {
        if x < self.width && y < self.height {
            let idx = y * self.width + x;
            if depth < self.depthbuffer[idx] {
                self.framebuffer[idx] = color;
                self.depthbuffer[idx] = depth;
            }
        }
    }

    pub fn draw_line(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, color: u32) {
        // Bresenham's line algorithm
        let mut x0 = x0 as isize;
        let mut y0 = y0 as isize;
        let x1 = x1 as isize;
        let y1 = y1 as isize;
        let dx = (x1 - x0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let dy = -(y1 - y0).abs();
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;
        loop {
            self.draw_pixel(x0 as usize, y0 as usize, color, 0.0);
            if x0 == x1 && y0 == y1 { break; }
            let e2 = 2 * err;
            if e2 >= dy { err += dy; x0 += sx; }
            if e2 <= dx { err += dx; y0 += sy; }
        }
    }

    pub fn draw_triangle(&mut self, v0: (f32, f32, f32), v1: (f32, f32, f32), v2: (f32, f32, f32), color0: u32, color1: u32, color2: u32) {
        // Simple barycentric rasterization with color interpolation
        let min_x = v0.0.min(v1.0).min(v2.0).max(0.0) as usize;
        let max_x = v0.0.max(v1.0).max(v2.0).min(self.width as f32 - 1.0) as usize;
        let min_y = v0.1.min(v1.1).min(v2.1).max(0.0) as usize;
        let max_y = v0.1.max(v1.1).max(v2.1).min(self.height as f32 - 1.0) as usize;
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let px = x as f32 + 0.5;
                let py = y as f32 + 0.5;
                let (w0, w1, w2) = barycentric(px, py, v0, v1, v2);
                if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
                    let depth = w0 * v0.2 + w1 * v1.2 + w2 * v2.2;
                    let color = interpolate_color(color0, color1, color2, w0, w1, w2);
                    self.draw_pixel(x, y, color, depth);
                }
            }
        }
    }
}

fn barycentric(px: f32, py: f32, v0: (f32, f32, f32), v1: (f32, f32, f32), v2: (f32, f32, f32)) -> (f32, f32, f32) {
    let denom = (v1.1 - v2.1) * (v0.0 - v2.0) + (v2.0 - v1.0) * (v0.1 - v2.1);
    let w0 = ((v1.1 - v2.1) * (px - v2.0) + (v2.0 - v1.0) * (py - v2.1)) / denom;
    let w1 = ((v2.1 - v0.1) * (px - v2.0) + (v0.0 - v2.0) * (py - v2.1)) / denom;
    let w2 = 1.0 - w0 - w1;
    (w0, w1, w2)
}

fn interpolate_color(c0: u32, c1: u32, c2: u32, w0: f32, w1: f32, w2: f32) -> u32 {
    let r = (w0 * (c0 & 0xFF) as f32 + w1 * (c1 & 0xFF) as f32 + w2 * (c2 & 0xFF) as f32) as u8;
    let g = (w0 * ((c0 >> 8) & 0xFF) as f32 + w1 * ((c1 >> 8) & 0xFF) as f32 + w2 * ((c2 >> 8) & 0xFF) as f32) as u8;
    let b = (w0 * ((c0 >> 16) & 0xFF) as f32 + w1 * ((c1 >> 16) & 0xFF) as f32 + w2 * ((c2 >> 16) & 0xFF) as f32) as u8;
    let a = (w0 * ((c0 >> 24) & 0xFF) as f32 + w1 * ((c1 >> 24) & 0xFF) as f32 + w2 * ((c2 >> 24) & 0xFF) as f32) as u8;
    ((a as u32) << 24) | ((b as u32) << 16) | ((g as u32) << 8) | (r as u32)
}

// Example RGBA8 color helper
pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> u32 {
    ((a as u32) << 24) | ((b as u32) << 16) | ((g as u32) << 8) | (r as u32)
}
