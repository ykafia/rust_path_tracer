use super::na::{Vector3};
use super::*;
use std::sync::Arc;
use std::f32::consts::PI;
use std::time::*;

#[derive(Clone)]
pub struct Sphere {
    pub center: Vector3<f32>,
    pub radius: f32,
    pub material : Material
}

impl Sphere {
    pub fn textured(x: f32, y: f32, z: f32,texture_path : &str, radius : f32, albedo : f32) -> Sphere {
       
        let texture = image::io::Reader::open(texture_path).unwrap().decode().unwrap();
        
        Sphere {
            center: Vector3::new(x, y, z),
            radius: radius,
            material : Material {
                albedo : albedo,
                emissive : Surface::Texture(
                    texture
                ),
                reflectivity : None
            }
        }
    }

    pub fn new(x: f32, y: f32, z: f32, color : Colors,radius : f32, albedo : f32) -> Sphere {        
        Sphere {
            center: Vector3::new(x, y, z),
            radius: radius,
            material : Material {
                albedo : albedo,
                emissive : Surface::Color(
                    color.value()
                ),
                reflectivity : None
            }
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
    fn get_color(&self, intersection : Vector3<f32> ) -> Color {
        self.material.emissive.color(
            self.get_texcoord(intersection)
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
            x : (PI + phi)/(PI*2.0),
            y : (PI + theta)/(PI*2.0)
        }
    }
    fn get_reflectivity(&self) -> Option<f32> {
        self.material.reflectivity
    }
}

impl ECSIntersectable for Sphere {
    fn simple_intersect_ecs(&self, ray: &Ray, transform : &TransformComponent) -> bool {
        let center = transform.position;
        let l = center - ray.origin;
        let adj = l.dot(&ray.direction);
        let d2 = l.dot(&l) - (adj * adj);
        let radius2 = self.radius * self.radius;
        d2 > radius2
    }
    fn intersect_ecs(&self, ray: &Ray, transform : &TransformComponent) -> Option<PointInfo> {
        let center = transform.position;
        let l = center - ray.origin;
        
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
    fn get_color_ecs(&self, intersection : Vector3<f32>, transform : &TransformComponent) -> Color {
        self.material.emissive.color(
            self.get_texcoord_ecs(intersection, transform)
        )
    }
    fn get_albedo_ecs(&self) -> f32{
        self.material.albedo
    }
    fn get_texcoord_ecs(&self, intersect : Vector3<f32>, transform : &TransformComponent) -> TexCoord {
        let center = transform.position;
        let spherical_coord = intersect - center;

        let phi  = spherical_coord.z.atan2(spherical_coord.x);
        let theta = (spherical_coord.y / self.radius).acos();
        TexCoord {
            x : (PI + phi)/(PI*2.0),
            y : (PI + theta)/(PI*2.0)
        }
    }
    fn get_reflectivity_ecs(&self) -> Option<f32> {
        self.material.reflectivity
    }
}
