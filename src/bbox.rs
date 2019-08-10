use cgmath::{vec3, Vector3, InnerSpace};
use crate::ray::Ray;
use cgmath::num_traits::float::FloatCore;
use crate::geometry::Intersect;
use crate::vec_utils::{vec3_min, vec3_max, max_component, vec3_inverse, vec3_abs, vec3_mul, vec3_sign, max, min};

#[derive(Copy, Clone, Debug)]
pub struct BBox {
    pub min: Vector3<f32>,
    pub max: Vector3<f32>,
}

pub fn enclosing_bbox(left: &BBox, right: &BBox) -> BBox {
    let min = vec3_min(left.min, right.min);
    let max = vec3_max(left.max, right.max);
    BBox {
        min: min,
        max: max,
    }
}

impl Intersect for BBox {
    fn intersect(&self, ray: &Ray) -> Option<f32> {
        let max_dist: f32 = f32::max_value();
        let min_dist: f32 = 0.0;

        //let sign = sign(ray.inverse_direction);
        let sign_x = ray.inverse_direction.x < 0.0;
        let sign_y = ray.inverse_direction.y < 0.0;
        let sign_z = ray.inverse_direction.z < 0.0;

        let bbox = if sign_x { self.max } else { self.min };
        let mut txmin = (bbox.x - ray.origin.x) * ray.inverse_direction.x;
        let bbox = if sign_x { self.min } else { self.max };
        let mut txmax = (bbox.x - ray.origin.x) * ray.inverse_direction.x;

        let bbox = if sign_y { self.max } else { self.min };
        let mut tymin = (bbox.y - ray.origin.y) * ray.inverse_direction.y;
        let bbox = if sign_y { self.min } else { self.max };
        let mut tymax = (bbox.y - ray.origin.y) * ray.inverse_direction.y;

        if txmin > tymax || tymin > txmax {
            return Option::None;
        }

        if tymin > txmin {
            txmin = tymin;
        }
        if tymax < txmax {
            txmax = tymax;
        }

        let bbox = if sign_z { self.max } else { self.min };
        let mut tzmin = (bbox.z - ray.origin.z) * ray.inverse_direction.z;
        let bbox = if sign_z { self.min } else { self.max };
        let mut tzmax = (bbox.z - ray.origin.z) * ray.inverse_direction.z;

        if txmin > tzmax || tzmin > txmax {
            return Option::None;
        }

        if tzmin > txmin {
            txmin = tzmin;
        }
        if tzmax < txmax {
            txmax = tzmax;
        }

        if txmin < max_dist && txmax > min_dist {
            return Option::Some(txmin)
        }

        return Option::None;
    }
}

impl BBox {
    fn center(&self) -> Vector3<f32> {
        0.5_f32 * (self.min + self.max)
    }

    fn extents(&self) -> Vector3<f32> {
        self.max - self.min
    }

    pub fn intersect_fast(&self, ray: &Ray) -> Option<f32> {
        let t1 = (self.min.x - ray.origin.x) * ray.inverse_direction.x;
        let t2 = (self.max.x - ray.origin.x) * ray.inverse_direction.x;
        let t3 = (self.min.y - ray.origin.y) * ray.inverse_direction.y;
        let t4 = (self.max.y - ray.origin.y) * ray.inverse_direction.y;
        let t5 = (self.min.z - ray.origin.z) * ray.inverse_direction.z;
        let t6 = (self.max.z - ray.origin.z) * ray.inverse_direction.z;

        let tmin = max(max(min(t1, t2), min(t3, t4)), min(t5, t6));
        let tmax = min(min(max(t1, t2), max(t3, t4)), max(t5, t6));

        if tmax < 0.0 {
            return Option::None;
        }

        if tmin > tmax {
            return Option::None;
        }

        return Option::Some(tmin);
    }

    pub fn intersect_faster(&self, ray: &Ray) -> f32 {
        let t1 = (self.min.x - ray.origin.x) * ray.inverse_direction.x;
        let t2 = (self.max.x - ray.origin.x) * ray.inverse_direction.x;
        let t3 = (self.min.y - ray.origin.y) * ray.inverse_direction.y;
        let t4 = (self.max.y - ray.origin.y) * ray.inverse_direction.y;
        let t5 = (self.min.z - ray.origin.z) * ray.inverse_direction.z;
        let t6 = (self.max.z - ray.origin.z) * ray.inverse_direction.z;

        let tmin = max(max(min(t1, t2), min(t3, t4)), min(t5, t6));
        let tmax = min(min(max(t1, t2), max(t3, t4)), max(t5, t6));

        if tmax < 0.0 {
            return f32::nan();
        }

        if tmin > tmax {
            return f32::nan();
        }

        return tmin;
    }
}

/*fn intersect_bbox_fast(bbox: &BBox, ray: &Ray) {
    let can_start_in_box = true;

    let origin = ray.origin - bbox.center();

    let winding = (if can_start_in_box && (max_component(vec3_mul(vec3_abs(ray.origin), (vec3_inverse(bbox.extents() )))) < 1.0) { -1.0 } else { 1.0 });
    let sgn = -vec3_sign(ray.direction);
    let d = vec3_mul(bbox.extents() * winding, sgn) - ray.origin;

    let test = Vector3
}*/