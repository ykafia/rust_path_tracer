use super::na::{Rotation3, Vector3};
use super::*;

pub struct Sphere {
    pub center: Vector3<f32>,
    pub color: Color,
    pub radius: f32,
}

impl Sphere {
    pub fn new(x: f32, y: f32, z: f32) -> Sphere {
        Sphere {
            center: Vector3::new(x, y, z),
            radius: 1.0,
            color: Color {
                r: 0,
                g: 125,
                b: 225,
                a: 0,
            },
        }
    }
    pub fn new_blue(x: f32, y: f32, z: f32) -> Sphere {
        Sphere {
            center: Vector3::new(x, y, z),
            radius: 1.0,
            color: Color {
                r: 0,
                g: 125,
                b: 225,
                a: 0,
            },
        }
    }
    pub fn new_red(x: f32, y: f32, z: f32) -> Sphere {
        Sphere {
            center: Vector3::new(x, y, z),
            radius: 1.0,
            color: Color {
                r: 225,
                g: 120,
                b: 120,
                a: 0,
            },
        }
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<f32> {
        let l = self.center - ray.origin;
        let adj = l.dot(&ray.direction);
        let d2 = l.dot(&l) - (adj * adj);
        let radius2 = self.radius * self.radius;
        if d2 > radius2 {
            return None;
        }
        let thc = (radius2 - d2).sqrt();
        let t0 = adj - thc;
        let t1 = adj + thc;

        if t0 < 0.0 && t1 < 0.0 {
            return None;
        }

        let distance = if t0 < t1 { t0 } else { t1 };
        Some(distance)
    }
}
