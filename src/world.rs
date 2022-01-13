use crate::{renderer::{camera::Camera, engine::GraphicsEngine}, input_manager::InputManager};

pub struct World<'a> {
    camera: Camera,
    graphics_engine: GraphicsEngine<'a>,
    pub input_manager: InputManager
}

impl<'a> World<'a> {
    pub fn new(graphics_engine: GraphicsEngine<'a>) -> Self {
        Self {
            camera: Camera::new(),
            graphics_engine,
            input_manager: InputManager::new()
        }
    }

    pub fn update(&mut self) {

    }

    pub fn event(&mut self) {

    }

    pub fn render(&mut self) {

    }
}
