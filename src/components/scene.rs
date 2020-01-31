use super::na::{Vector3};
use super::*;


pub struct Scene {
    pub width : u32,
    pub height : u32,
    pub fov : f32,
    pub camera : Camera,
    pub elements :  Vec<Box<dyn Intersectable>>
}

impl Scene {
    pub fn new() -> Scene {
        Scene{
            width : 600,
            height : 400,
            fov : 90.0,
            camera : Camera::new(),
            elements : 
                    vec!(   
                        Box::new(Sphere::new_blue(0f32,0f32,-5f32)),
                        Box::new(Sphere::new_red(1f32,1f32,-6f32)),
                        Box::new(Plane::new())
                    )
        }
    }
    pub fn fire_rays(&self, image : &mut DynamicImage) -> DynamicImage {
        let mut temp : (Color, f32) = (Color::new(0,0,0,0),std::f32::MAX);
        for x in 0..self.width {
            for y in 0..self.height {
                for element in &self.elements {
                    let ray = Ray::new(x, y, self);
                    //TODO : recheck the distance thingy
                    match element.intersect(&ray){
                        Some(d) => if d<temp.1 {
                            temp.1 = d;
                            temp.0 = element.get_color();
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