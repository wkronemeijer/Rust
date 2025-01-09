use glium::backend::Facade;

pub struct TextRenderer {}

impl TextRenderer {
    pub fn new(_: &impl Facade) -> crate::Result<Self> { Ok(TextRenderer {}) }
}
