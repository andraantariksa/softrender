use std::collections::HashSet;
use winit::event::{KeyboardInput, VirtualKeyCode, WindowEvent};

pub struct InputManager {
    key_pressed: HashSet<VirtualKeyCode>,
}

impl InputManager {
    pub fn new() -> Self {
        Self {
            key_pressed: HashSet::new()
        }
    }

    pub fn process(&mut self, window_event: &WindowEvent) {
        match window_event {
            WindowEvent::KeyboardInput { input, .. } => {
                let vkc = input.virtual_keycode.unwrap();
                match input.state {
                    winit::event::ElementState::Pressed => {
                        self.key_pressed.insert(vkc);
                    },
                    winit::event::ElementState::Released => {
                        self.key_pressed.remove(&vkc);
                    },
                }
            },
            // WindowEvent::ModifiersChanged(_) => todo!(),
            // WindowEvent::CursorMoved { device_id, position, modifiers } => todo!(),
            // WindowEvent::CursorEntered { device_id } => todo!(),
            // WindowEvent::CursorLeft { device_id } => todo!(),
            // WindowEvent::MouseWheel { device_id, delta, phase, modifiers } => todo!(),
            // WindowEvent::MouseInput { device_id, state, button, modifiers } => todo!(),
            _ => {}
        }
    }
}
