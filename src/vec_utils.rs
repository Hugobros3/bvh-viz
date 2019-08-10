use cgmath::Vector3;
use std::ops::Mul;
use cgmath::num_traits::real::Real;

#[inline(always)]
pub fn min<S: PartialOrd>(left: S, right: S) -> S {
    if left < right { left } else { right }
}

#[inline(always)]
pub fn vec3_min<S>(left: Vector3<S>, right: Vector3<S>) -> Vector3<S>
    where S: PartialOrd {
    Vector3 {
        x: min(left.x, right.x),
        y: min(left.y, right.y),
        z: min(left.z, right.z),
    }
}

#[inline(always)]
pub fn max<S: PartialOrd>(left: S, right: S) -> S {
    if left > right { left } else { right }
}

#[inline(always)]
pub fn vec3_max<S>(left: Vector3<S>, right: Vector3<S>) -> Vector3<S>
    where S: PartialOrd {
    Vector3 {
        x: max(left.x, right.x),
        y: max(left.y, right.y),
        z: max(left.z, right.z),
    }
}

#[inline(always)]
pub fn vec3_sign(v: Vector3<f32>) -> Vector3<f32> {
    Vector3 { x: v.x.signum(), y: v.y.signum(), z: v.y.signum() }
}

#[inline(always)]
pub fn vec3_abs(v: Vector3<f32>) -> Vector3<f32> {
    Vector3 { x: v.x.abs(), y: v.y.abs(), z: v.y.abs() }
}

#[inline(always)]
pub fn vec3_inverse(v: Vector3<f32>) -> Vector3<f32> {
    Vector3 {
        x: 1.0 / v.x,
        y: 1.0 / v.y,
        z: 1.0 / v.z,
    }
}

#[inline(always)]
pub fn vec3_mul<S>(left: Vector3<S>, right: Vector3<S>) -> Vector3<S>
    where S: Real + Mul {
    Vector3 {
        x: left.x * right.x,
        y: left.y * right.y,
        z: left.z * right.z,
    }
}

#[inline(always)]
pub fn max_component(v: Vector3<f32>) -> f32 {
    let m = max(v.x, v.y);
    return max(m, v.z);
}