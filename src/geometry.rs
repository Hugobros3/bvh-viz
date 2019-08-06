use cgmath::{vec3, Vector3, InnerSpace};
use crate::ray::Ray;

#[derive(Copy, Clone)]
pub struct Triangle {
    pub v0: Vector3<f32>,
    pub v1: Vector3<f32>,
    pub v2: Vector3<f32>,
}

pub trait Intersect {
    fn intersect(&self, ray: &Ray) -> Option<f32>;
}

impl Default for Triangle {
    fn default() -> Self {
        degenerate_triangle()
    }
}

pub fn degenerate_triangle() -> Triangle {
    let zero = vec3(0.0, 0.0, 0.0);
    Triangle {
        v0: zero,
        v1: zero,
        v2: zero,
    }
}

const EPSILON: f32 = 0.000001;

impl Intersect for Triangle {
    fn intersect(&self, ray: &Ray) -> Option<f32> {
        let e1 = self.v1 - self.v0;
        let e2 = self.v2 - self.v0;

        let h = ray.direction.cross(e2);
        let a = e1.dot(h);
        if a > -EPSILON && a < EPSILON {
            return Option::None;
        }
        let f = 1.0_f32 / a;
        let s = ray.origin - self.v0;
        let u = f * s.dot(h);
        if u < 0.0 || u > 1.0 {
            return Option::None;
        }

        let q = s.cross(e1);
        let v = f * ray.direction.dot(q);
        if v < 0.0 || u + v > 1.0 {
            return Option::None
        }

        let t = f * e2.dot(q);
        if t > EPSILON {
            return Option::Some(t);
        } else {
            return Option::None;
        }
    }
}