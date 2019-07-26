use cgmath::{vec3, Vector3, InnerSpace};

#[derive(Copy, Clone)]
pub struct Triangle {
    v0: Vector3<f32>,
    v1: Vector3<f32>,
    v2: Vector3<f32>,
}

pub fn degenerate_triangle() -> Triangle {
    let zero = vec3(0.0, 0.0, 0.0);
    Triangle {
        v0: zero,
        v1: zero,
        v2: zero,
    }
}