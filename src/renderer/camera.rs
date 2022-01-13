use nalgebra::{Point3, Perspective3, Matrix4, Translation3, Rotation3, Vector3};

use crate::input_manager::InputManager;

pub struct Camera {
    position: Point3<f32>,
    yaw: f32,
    pitch: f32,
    fovy: f32,
    znear: f32,
    zfar: f32
}

impl Camera {
    pub fn new() -> Self {
        Self {
            position: Point3::new(0.0, 0.0, -5.0),
            yaw: 0.0,
            pitch: 0.0,
            fovy: 3.14 / 4.0,
            zfar: 1000.0,
            znear: 0.1
        }
    }

    pub fn process_event(&mut self, input_manager: &InputManager) {
        
    }

    pub fn to_matrix(&self) -> Matrix4<f32> {
        let view_matrix = Translation3::new(self.position.x, self.position.y, self.position.z).to_homogeneous();
            * &Rotation3::from_axis_angle(&Vector3::y_axis(), self.yaw).to_homogeneous()
            * Rotation3::from_axis_angle(&Vector3::x_axis(), self.pitch).to_homogeneous();
        Perspective3::new(16.0 / 9.0, self.fovy, self.znear, self.zfar).to_homogeneous() * view_matrix
    }
}
