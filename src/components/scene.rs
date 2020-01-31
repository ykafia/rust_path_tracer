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
            spheres : vec!(Sphere::new_blue(0f32,0f32,-5f32),Sphere::new_red(1f32,1f32,-6f32))
        }
    }
    pub fn fire_rays(&self, image : &mut DynamicImage) -> DynamicImage {
        let mut temp : (Color, f32) = (Color::new(0,0,0,0),std::f32::MAX);
        for x in 0..self.width {
            for y in 0..self.height {
                for sphere in &self.spheres {
                    let ray = Ray::new(x, y, self);
                    //Check if the distance is right or not
                    match sphere.intersect(&ray){
                        Some(d) => if d<temp.1 {
                            temp.1 = d;
                            temp.0 = sphere.color.clone();
                        },
                        None => ()
                    } 
                }
                image.put_pixel(x, y, temp.0.to_rgba());
                temp = (Color::new(0,0,0,0),std::f32::MAX);
                
            }
        }
        image.clone()
    }
}