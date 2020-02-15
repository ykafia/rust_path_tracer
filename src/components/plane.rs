use super::na::{Vector3};
use super::*;

#[derive(Copy,Clone)]
pub struct Plane {
    pub position: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub color: Color,
    pub albedo : f32,
}

impl Plane {
    pub fn new() -> Plane {
        Plane {
            position: Vector3::new(0f32, -0.5f32, 0f32),
            normal: Vector3::new(0f32, -1f32, 0f32),
            color: Colors::GREY.value(),
            albedo : 1.0
        }
    }
}

impl Intersectable for Plane {
    fn simple_intersect(&self, ray: &Ray) -> bool {
        let normal = &self.normal;
        let denom = normal.dot(&ray.direction);
        denom > 1e-6
    }
    fn intersect(&self, ray: &Ray) -> Option<PointInfo> {
        let normal = &self.normal;
        let denom = normal.dot(&ray.direction);
        if denom > 1e-6 {
            let v = self.position - ray.origin;
            let distance = v.dot(&normal) / denom;
            if distance >= 0.0 {
                return Some(
                    PointInfo {
                        distance : distance,
                        normal : -self.normal,
                        intersection : ray.origin + ray.direction * distance
                    }
                );
            }
        }
        None
    }
    fn get_color(&self) -> Color {
        self.color.clone()
    }
    fn get_position(&self) -> Vector3<f32> {
        self.position
    }
    fn get_albedo(&self) -> f32 {
        self.albedo
    }
}
