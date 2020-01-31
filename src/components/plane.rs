use super::na::{Vector3,Rotation3};
use super::*;




pub struct Plane {
    pub position : Vector3<f32>,
    pub normal : Vector3<f32>,
    pub color : Color
}

impl Plane{
    pub fn new() -> Plane {
        Plane {
            position : Vector3::new(0f32,0f32,0f32),
            normal : Vector3::new(0f32,-1f32,0f32),
            color : Color::new(100, 100, 100, 100)
        }
    }
}

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<f32> {
        let normal = &self.normal;
        let denom = normal.dot(&ray.direction);
        if denom > 1e-6 {
            let v = self.position - ray.origin;
            let distance = v.dot(&normal) / denom;
            if distance >= 0.0 {
                return Some(distance);
            }
        }
        None
    }
    fn get_color(&self) -> Color {
        self.color.clone()
    }
}

