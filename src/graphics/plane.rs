use super::na::{Vector3};
use super::*;

#[derive(Clone)]
pub struct Plane {
    pub position: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub material : Material,
    pub repeat_texture : Option<f32>
}

impl Plane {
    pub fn new() -> Plane {
        Plane {
            position: Vector3::new(0f32, -0.5f32, 0f32),
            normal: Vector3::new(0f32, -1f32, 0f32),
            material : Material {
                emissive : Surface::Color(Colors::GREY.value()),
                albedo : 1.0,
                reflectivity : Some(0.5)
            },
            repeat_texture : None
        }
    }
    pub fn textured() -> Plane {
        let texture = 
            image::io::Reader::open("textures/checker.png")
            .unwrap()
            .decode()
            .unwrap();
        Plane {
            position: Vector3::new(0f32, -0.5f32, 0f32),
            normal: Vector3::new(0f32, -1f32, 0f32),
            material : Material {
                albedo : 1.0,
                emissive : Surface::Texture(
                    texture
                ),
                reflectivity : Some(0.1)             
            },
            repeat_texture : Some(10.0)
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
        self.material.emissive.color(
            self.get_texcoord(intersection)
        )
    }
    fn get_position(&self) -> Vector3<f32> {
        self.position
    }
    fn get_albedo(&self) -> f32 {
        self.material.albedo
    }
    fn get_texcoord(&self, intersect : Vector3<f32>) -> TexCoord {
        let mut x_axis : Vector3<f32> = self.normal.cross(&Vector3::z()).normalize();
        
        if vector_length(x_axis) == 0.0 {
            x_axis = self.normal.cross(&Vector3::y());
        }
        
        let y_axis : Vector3<f32> = self.normal.cross(&x_axis).normalize();

        let plane_point = intersect - self.position;
        // println!("plane u : {}\nplane v : {}",plane_point.dot(&x_axis),plane_point.dot(&y_axis));
        let repeat = self.repeat_texture.unwrap_or(10.0);
        let u = plane_point.dot(&x_axis).rem_euclid(repeat);
        let v = plane_point.dot(&y_axis).rem_euclid(repeat);
        TexCoord {
            x : u/repeat,
            y : v/repeat
        }
    }
    fn get_reflectivity(&self) -> Option<f32> {
        self.material.reflectivity
    }
}

impl ECSIntersectable for Plane {
    fn simple_intersect_ecs(&self, ray: &Ray, transform : &TransformComponent) -> bool {
        let normal = &self.normal;
        let denom = normal.dot(&ray.direction);
        if denom > 1e-6 {
            let p0l0 = transform.position - ray.origin;
            let t = p0l0.dot(&normal) / denom;
            t >= 0.0
        } else {
            false
        }
        
    }
    fn intersect_ecs(&self, ray: &Ray, transform : &TransformComponent) -> Option<PointInfo> {
        let normal = &self.normal;
        let denom = normal.dot(&ray.direction);
        if denom > 1e-6 {
            let v = transform.position - ray.origin;
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
    fn get_color_ecs(&self, intersection : Vector3<f32>, transform : &TransformComponent) -> Color {
        self.material.emissive.color(
            self.get_texcoord_ecs(intersection, transform)
        )
    }
    fn get_albedo_ecs(&self) -> f32 {
        self.material.albedo
    }
    fn get_texcoord_ecs(&self, intersect : Vector3<f32>, transform : &TransformComponent) -> TexCoord {
        let mut x_axis : Vector3<f32> = self.normal.cross(&Vector3::z()).normalize();
        
        if vector_length(x_axis) == 0.0 {
            x_axis = self.normal.cross(&Vector3::y());
        }
        
        let y_axis : Vector3<f32> = self.normal.cross(&x_axis).normalize();

        let plane_point = intersect - transform.position;
        // println!("plane u : {}\nplane v : {}",plane_point.dot(&x_axis),plane_point.dot(&y_axis));
        let repeat = self.repeat_texture.unwrap_or(10.0);
        let u = plane_point.dot(&x_axis).rem_euclid(repeat);
        let v = plane_point.dot(&y_axis).rem_euclid(repeat);
        TexCoord {
            x : u/repeat,
            y : v/repeat
        }
    }
    fn get_reflectivity_ecs(&self) -> Option<f32> {
        self.material.reflectivity
    }
}


