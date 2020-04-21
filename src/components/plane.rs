use super::na::{Vector3};
use super::*;

#[derive(Copy,Clone,Debug)]
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
        if denom > 1e-6 {
            let p0l0 = self.position - ray.origin;
            let t = p0l0.dot(&normal) / denom;
            t >= 0.0
        } else {
            false
        }
        
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
    fn get_color(&self, intersection : Vector3<f32>) -> Color {
        self.color.clone()
    }
    fn get_position(&self) -> Vector3<f32> {
        self.position
    }
    fn get_albedo(&self) -> f32 {
        self.albedo
    }
    fn get_texcoord(&self, intersect : Vector3<f32>) -> TexCoord {
        let mut x_axis : Vector3<f32> = self.normal.cross(&Vector3::z());
        
        if vector_length(x_axis) == 0.0 {
            x_axis = self.normal.cross(&Vector3::y());
        }
        
        let y_axis : Vector3<f32> = self.normal.cross(&x_axis);

        let plane_point = intersect - self.position;
        TexCoord {
            x : plane_point.dot(&x_axis),
            y : plane_point.dot(&y_axis)
        }
    }
}


