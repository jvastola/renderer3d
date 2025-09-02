// Resource destruction trait
pub trait Destroy {
    fn destroy(&mut self);
}
// Placeholder for future texture support
pub trait Texture {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn format(&self) -> &str;
}
// Placeholder for future framebuffer support
pub trait Framebuffer {
    fn bind(&self) -> GfxResult<()>;
    fn unbind(&self) -> GfxResult<()>;
}
// Initial custom graphics API traits and dummy implementations

use std::error::Error;
use std::fmt;

pub type GfxResult<T> = Result<T, GfxError>;

#[derive(Debug)]
pub struct GfxError {
    pub message: String,
}

impl fmt::Display for GfxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GfxError: {}", self.message)
    }
}

impl Error for GfxError {}

pub trait Device {
    fn create_buffer(&self, size: usize) -> GfxResult<Box<dyn Buffer>>;
    fn create_shader(&self, source: &str) -> GfxResult<Box<dyn Shader>>;
    fn create_pipeline(&self, shader: &dyn Shader) -> GfxResult<Box<dyn Pipeline>>;
    fn submit(&self, queue: &dyn CommandQueue) -> GfxResult<()>;
}

pub trait Buffer {
    fn write(&mut self, data: &[u8]) -> GfxResult<()>;
    fn size(&self) -> usize;
}

pub trait Shader {
    fn set_source(&mut self, source: &str) -> GfxResult<()>;
    fn get_source(&self) -> &str;
}

pub trait Pipeline {
    fn bind(&self) -> GfxResult<()>;
    fn is_bound(&self) -> bool;
}

pub trait CommandQueue {
    fn add_command(&mut self, command: &str) -> GfxResult<()>;
    fn clear(&mut self);
    fn commands(&self) -> &[String];
}

// Dummy implementations
pub struct DummyDevice;
impl Device for DummyDevice {
    fn create_buffer(&self, size: usize) -> GfxResult<Box<dyn Buffer>> {
        Ok(Box::new(DummyBuffer { data: vec![0; size] }))
    }
    fn create_shader(&self, source: &str) -> GfxResult<Box<dyn Shader>> {
        Ok(Box::new(DummyShader { source: source.to_string() }))
    }
    fn create_pipeline(&self, _shader: &dyn Shader) -> GfxResult<Box<dyn Pipeline>> {
        Ok(Box::new(DummyPipeline { bound: false }))
    }
    fn submit(&self, _queue: &dyn CommandQueue) -> GfxResult<()> {
        println!("DummyDevice: submit called");
        Ok(())
    }
}

pub struct DummyBuffer {
    data: Vec<u8>,
}
impl Buffer for DummyBuffer {
    fn write(&mut self, data: &[u8]) -> GfxResult<()> {
        if data.len() > self.data.len() {
            return Err(GfxError { message: "Buffer overflow".to_string() });
        }
        self.data[..data.len()].copy_from_slice(data);
        println!("DummyBuffer: write called, size {}", data.len());
        Ok(())
    }
    fn size(&self) -> usize {
        self.data.len()
    }
}
impl Destroy for DummyBuffer {
    fn destroy(&mut self) {
        println!("DummyBuffer: destroyed");
        self.data.clear();
    }
}

pub struct DummyShader {
    source: String,
}
impl Shader for DummyShader {
    fn set_source(&mut self, source: &str) -> GfxResult<()> {
        if source.trim().is_empty() {
            return Err(GfxError { message: "Shader source is empty".to_string() });
        }
        self.source = source.to_string();
        println!("DummyShader: set_source called");
        Ok(())
    }
    fn get_source(&self) -> &str {
        &self.source
    }
}
impl Destroy for DummyShader {
    fn destroy(&mut self) {
        println!("DummyShader: destroyed");
        self.source.clear();
    }
}

pub struct DummyPipeline {
    bound: bool,
}
impl Pipeline for DummyPipeline {
    fn bind(&self) -> GfxResult<()> {
        println!("DummyPipeline: bind called");
        Ok(())
    }
    fn is_bound(&self) -> bool {
        self.bound
    }
}
impl Destroy for DummyPipeline {
    fn destroy(&mut self) {
        println!("DummyPipeline: destroyed");
    }
}

pub struct DummyCommandQueue {
    commands: Vec<String>,
}
impl DummyCommandQueue {
    pub fn new() -> Self {
        Self { commands: Vec::new() }
    }
}
impl CommandQueue for DummyCommandQueue {
    fn add_command(&mut self, command: &str) -> GfxResult<()> {
        if command.trim().is_empty() {
            return Err(GfxError { message: "Command is empty".to_string() });
        }
        self.commands.push(command.to_string());
        println!("DummyCommandQueue: add_command called");
        Ok(())
    }
    fn clear(&mut self) {
        println!("DummyCommandQueue: cleared");
        self.commands.clear();
    }
    fn commands(&self) -> &[String] {
        &self.commands
    }
}
impl Destroy for DummyCommandQueue {
    fn destroy(&mut self) {
        println!("DummyCommandQueue: destroyed");
        self.commands.clear();
    }
}
