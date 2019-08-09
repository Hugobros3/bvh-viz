use cgmath::{Vector3, Vector2, vec3};
use crate::camera::Camera;
use minifb::Window;

pub struct Controller {
    pub position: Vector3<f32>,
    pub rotation: Vector2<f32>,
}

impl Controller {
    pub fn new(position: Vector3<f32>, rotation: Vector2<f32>) -> Self {
        Controller { position, rotation }
    }

    pub fn update(&mut self, window: &Window) {

    }

    pub fn to_camera(&self, window: &Window) -> Camera {
        let view_dir = Vector3 { x: f32::sin(self.rotation.x) * f32::cos (self.rotation.y), y: f32::sin(self.rotation.y), z: f32::cos(self.rotation.x) * f32::cos(self.rotation.y) };
        let ratio = window.get_size().0 as f32 / window.get_size().1 as f32;
        Camera::new(self.position, view_dir, up, ratio, 65.0f32)
    }
}

const up: Vector3<f32> = vec3(0.0, 1.0, 0.0);