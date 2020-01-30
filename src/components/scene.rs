use super::na::{Vector3};
use super::*;


pub struct Scene {
    pub width : u32,
    pub height : u32,
    pub fov : f32,
    
    pub spheres :  Vec<Sphere>
}

impl Scene {
    pub fn new() -> Scene {
        Scene{
            width : 600,
            height : 400,
            fov : 90.0,
            spheres : vec!(Sphere::new(0f32,0f32,-5f32),Sphere::new(1f32,1f32,-6f32))
        }
    }
    pub fn fire_rays(&self, image : &mut DynamicImage) -> DynamicImage {
        for x in 0..self.width {
            for y in 0..self.height {
                for sphere in &self.spheres {
                    let ray = Ray::new(x, y, self);
    
                    if sphere.intersect(&ray) {
                        image.put_pixel(x, y, sphere.color.to_rgba())
                    } 
                }
                
            }
        }
        image.clone()
    }
}