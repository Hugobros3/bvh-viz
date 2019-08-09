use cgmath::{Vector3, InnerSpace};
use crate::ray::Ray;
use cgmath::num_traits::float::FloatCore;

pub struct Camera {
    pub eye: Vector3<f32>,
    pub view_direction: Vector3<f32>,
    pub up: Vector3<f32>,
    pub ratio: f32,
    pub fov: f32,
    lower_left_corner: Vector3<f32>,
    horizontal: Vector3<f32>,
    vertical: Vector3<f32>,
}

const PI: f32 = std::f32::consts::PI;

impl Camera {
    pub fn new(eye: Vector3<f32>, view_direction: Vector3<f32>, up: Vector3<f32>, ratio: f32, fov: f32) -> Self {
        let theta = fov * PI / 180.0;
        let half_height = f32::tan(theta / 2.0);
        let half_width = ratio * half_height;

        let w = view_direction;
        let u = up.cross(w).normalize();
        let v = w.cross(u);

        let lower_left_corner = ((eye - (u * half_width)) - (v * half_height)) - view_direction;
        let horizontal = u * half_width * 2.0;
        let vertical = v * half_height * 2.0;

        Camera { eye, view_direction, up, ratio, fov, lower_left_corner, horizontal, vertical }
    }

    pub fn make_ray(&self, s: f32, t: f32) -> Ray {
        let dir = self.lower_left_corner + (self.horizontal * s) + (self.vertical * t) - self.eye;
        Ray {
            origin: self.eye,
            direction: dir,
            inverse_direction: inverse(dir),
            t_max: f32::max_value(),
            hit_point: None,
            steps: 0
        }
    }
}

fn inverse(v: Vector3<f32>) -> Vector3<f32> {
    Vector3 {
        x: 1.0 / v.x,
        y: 1.0 / v.y,
        z: 1.0 / v.z,
    }
}