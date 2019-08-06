use cgmath::{vec3, Vector3, InnerSpace};

pub struct Ray {
    pub origin: Vector3<f32>,
    pub direction: Vector3<f32>,
    pub inverse_direction: Vector3<f32>,
    pub t_max: f32,
    pub hit_point: Option<HitPoint>,
    pub steps: i32,
}

pub struct HitPoint {
    pub position: Vector3<f32>
}