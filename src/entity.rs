// ...existing code...
use crate::line_renderer::LineVertex;

pub trait Entity {
    fn update(&mut self, dt: f32);
    /// Returns line vertices to be rendered for this entity
    fn line_vertices(&self) -> Vec<LineVertex>;
}

pub struct EntityManager {
    pub entities: Vec<Box<dyn Entity>>,
}

impl EntityManager {
    pub fn new() -> Self {
        Self { entities: Vec::new() }
    }
    pub fn add_entity<E: Entity + 'static>(&mut self, entity: E) {
        self.entities.push(Box::new(entity));
    }
    pub fn update_all(&mut self, dt: f32) {
        for entity in &mut self.entities {
            entity.update(dt);
        }
    }
    // Removed: render_all (no longer needed)
    pub fn collect_all_lines(&self) -> Vec<LineVertex> {
        let mut all_lines = Vec::new();
        for entity in &self.entities {
            all_lines.extend(entity.line_vertices());
        }
        all_lines
    }
}
