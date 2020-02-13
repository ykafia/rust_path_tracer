use super::na::Vector3;
use super::*;
use std::f32::consts::PI;


pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f32,
    pub camera: Camera,
    pub elements: Vec<Box<dyn Intersectable>>,
    pub lights : Vec<DirectionalLight>
}

impl Scene {
    pub fn new(camera_pos : Vector3<f32>) -> Scene {
        let mut result = Scene {
            width: 600,
            height: 400,
            fov: 90.0,
            camera: Camera::new(
                // position
                camera_pos,
                // direction
                Vector3::new(0f32, 0f32, 0f32).normalize()
            ),
            elements: vec![
                Box::new(Sphere::new(0f32, 0f32, -3f32,Colors::BLUE,1.0)),
                Box::new(Sphere::new(0f32, 1f32, -4f32,Colors::RED,1.0)),
                Box::new(Sphere::new(1f32, 1f32, -1f32,Colors::GREEN,1.0)),
                Box::new(Plane::new()),
            ],
            lights : vec![
                DirectionalLight {
                    direction : Vector3::new(0f32,-1f32,-1f32),
                    color : Colors::WHITE.value(),
                    intensity : 0.5
                },
                DirectionalLight {
                    direction : Vector3::new(0f32,-1f32,1f32),
                    color : Colors::WHITE.value(),
                    intensity : 0.5
                }
            ]
            
        };
        result.camera.change_rotation(
            result.elements.first().expect("a vector").get_position() - camera_pos
        );
        result
    }
    pub fn fire_rays(&self, image: &mut DynamicImage) -> DynamicImage {
        let mut temp: (Color, f32) = (Color::new(0.0, 0.0, 0.0, 0.0), std::f32::MAX);
        for x in 0..self.width {
            for y in 0..self.height {
                for element in &self.elements {
                    let ray = Ray::from_camera(x, y, self);
                    match element.intersect(&ray) {
                        Some(d) => {
                            if d.distance < temp.1 {
                                let mut color = Colors::BLACK.value();
                                for light in &self.lights {
                                    let intensity = d.normal.dot(&(-light.direction)).max(0.0) * light.intensity;
                                    let reflected = element.get_albedo();
                                    let absorbed = Colors::WHITE.value() - element.get_color();
                                    let final_color = light.color.clone() - absorbed;
                                    let shadowed = self.is_shadowed(
                                        &Ray {
                                            origin : d.intersection + 1e-5 * d.normal,
                                            direction  : -light.direction.normalize()
                                        },
                                    );
                                    color = color + match shadowed {
                                        false => final_color  * intensity * reflected,
                                        true => final_color  * 0.0 * reflected,
                                    };

                                }
                                
                                temp.1 = d.distance;
                                temp.0  = color;
                               
                            }
                        }
                        None => (),
                    }
                }
                image.put_pixel(x, y, temp.0.to_rgba());
                temp = (Colors::SKYBLUE.value(), std::f32::MAX);
            }
        }
        image.clone()
    }
    pub fn is_shadowed(&self, ray : &Ray) -> bool {
        let mut result = false;
        for element in &self.elements {
            match element.intersect(ray) {
                Some(_) => result = true,
                _ => ()
            }
        }
        result
    }
}

pub struct DirectionalLight {
    direction : Vector3<f32>,
    color : Color,
    intensity : f32
}
