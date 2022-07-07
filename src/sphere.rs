use crate::{
    ray::Ray,
    vec3::{dot, Vec3},
};

pub fn hit_sphere(center: Vec3, radius: f64, r: &Ray) -> bool {
    let oc = &r.origin - &center;

    let a = dot(&r.direction, &r.direction);
    let b = dot(&oc, &r.direction) * 2.0;
    let c = dot(&oc, &oc) - radius * radius;

    let discriminant = b * b - a * c * 4.0;

    discriminant > 0.0
}
