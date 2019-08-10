use cgmath::{Vector3, Vector2, vec3};
use crate::camera::Camera;
use minifb::{Window, MouseButton, MouseMode};

pub struct Controller {
    pub position: Vector3<f32>,
    pub rotation: Vector2<f32>,
    last_mouse_x: i32,
    last_mouse_y: i32,
}

impl Controller {
    pub fn new(position: Vector3<f32>, rotation: Vector2<f32>) -> Self {
        Controller { position, rotation, last_mouse_x: -1, last_mouse_y: -1 }
    }

    pub fn update(&mut self, window: &Window) {
        if window.get_mouse_down(MouseButton::Left) {
            let mouse_pos = window.get_mouse_pos(MouseMode::Pass).expect("no mouse???");
            let mx = mouse_pos.0 as i32;
            let my = mouse_pos.1 as i32;

            let lx = self.last_mouse_x;
            let ly = self.last_mouse_y;

            if lx != -1 {
                let dx = mx - lx;
                let dy = my - ly;

                let rot_speed = 0.0125_f32;
                self.rotation.x -= dx as f32 * rot_speed;
                self.rotation.y += dy as f32 * rot_speed;
            }

            self.last_mouse_x = mx;
            self.last_mouse_y = my;
        } else {
            self.last_mouse_x = -1;
            self.last_mouse_y = -1;
        }
    }

    pub fn to_camera(&self, window: &Window) -> Camera {
        let view_dir = Vector3 { x: f32::sin(self.rotation.x) * f32::cos (self.rotation.y), y: f32::sin(self.rotation.y), z: f32::cos(self.rotation.x) * f32::cos(self.rotation.y) };
        let ratio = window.get_size().0 as f32 / window.get_size().1 as f32;
        Camera::new(self.position, view_dir, UP, ratio, 65.0f32)
    }
}

const UP: Vector3<f32> = vec3(0.0, 1.0, 0.0);