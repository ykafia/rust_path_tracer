use super::na::{Rotation3, Vector3};
use super::*;

#[derive(Copy,Clone)]
pub struct Sphere {
    pub center: Vector3<f32>,
    pub color: Color,
    pub radius: f32,
    pub albedo : f32,
}

impl Sphere {
    pub fn new(x: f32, y: f32, z: f32, color : Colors,albedo : f32) -> Sphere {
        Sphere {
            center: Vector3::new(x, y, z),
            radius: 1.0,
            color: color.value(),
            albedo : albedo
        }
    }
    
}

impl Intersectable for Sphere {
    fn simple_intersect(&self, ray: &Ray) -> bool {
        let l = self.center - ray.origin;
        let adj = l.dot(&ray.direction);
        let d2 = l.dot(&l) - (adj * adj);
        let radius2 = self.radius * self.radius;
        d2 > radius2
    }
    fn intersect(&self, ray: &Ray) -> Option<PointInfo> {
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
        let hitpoint = (distance * &ray.direction) + &ray.origin;
        let normal = &hitpoint-self.center;
        Some(
            PointInfo{
                distance : distance,
                normal : normal.normalize(),
                intersection : hitpoint
            }
        )
    }
    fn get_color(&self) -> Color {
        self.color.clone()
    }
    fn get_position(&self) -> Vector3<f32> {
        self.center
    }
    fn get_albedo(&self) -> f32{
        self.albedo
    }
}
