use super::na::{Vector3};
use super::*;

#[derive(Copy,Clone,Debug)]
pub struct Sphere<'a> {
    pub center: Vector3<f32>,
    pub radius: f32,
    pub material : Material<'a>
}

impl<'a> Sphere<'a> {
    pub fn new(x: f32, y: f32, z: f32, radius : f32, color : Colors,albedo : f32) -> Sphere<'a> {
        Sphere {
            center: Vector3::new(x, y, z),
            radius: radius,
            material : Material {
                albedo : albedo,
                emissive : Emissive::Color(color.value())
            }
        }
    }
    
}

impl<'a> Intersectable for Sphere<'a> {
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
    fn get_color(&self, intersection : Vector3<f32> ) -> Color {
        self.material.emissive.color(
            &self.get_texcoord(intersection)
        )
    }
    fn get_position(&self) -> Vector3<f32> {
        self.center
    }
    fn get_albedo(&self) -> f32{
        self.material.albedo
    }
    fn get_texcoord(&self, intersect : Vector3<f32>) -> TexCoord {
        let spherical_coord = intersect - self.center;
        let phi  = spherical_coord.z.atan2(spherical_coord.x);
        let theta = (spherical_coord.y / self.radius).acos();
        TexCoord {
            x : ((1.0+phi) / std::f32::consts::PI)/2.0,
            y : theta
        }
    }
}
