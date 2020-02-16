use super::na::Vector3;
use super::*;
use std::f32::consts::PI;

#[derive(Copy, Clone)]
pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f32,
    pub camera: Camera,
}

impl Scene {
    pub fn new(camera_pos: Vector3<f32>, target: Vector3<f32>) -> Scene {
        Scene {
            width: 600,
            height: 400,
            fov: 90.0,
            camera: Camera::new(
                // position
                camera_pos,
                // direction
                target.normalize(),
            ),
        }
    }
    pub fn fire_rays(
        &self,
        image: &mut DynamicImage,
        elements: &[Element],
        lights: &[Light],
    ) -> DynamicImage {
        let mut temp: (Color, f32) = (Color::new(0.0, 0.0, 0.0, 0.0), std::f32::MAX);
        for x in 0..self.width {
            for y in 0..self.height {
                for element in elements {
                    let ray = Ray::from_camera(x, y, self);
                    match element.intersect(&ray) {
                        Some(d) => {
                            if d.distance < temp.1 {
                                let mut color = Colors::BLACK.value();
                                for light in lights {
                                    let intensity = d.normal.dot(&(-light.get_direction(element))).max(0.0)
                                        * light.get_intensity(d.intersection);
                                    let reflected = element.get_albedo() / PI;
                                    // let absorbed = Colors::WHITE.value() - element.get_color();
                                    // let final_color = light.color.clone() - absorbed;
                                    let shadowed = self.is_shadowed(&Ray {
                                        origin: d.intersection + 1e-5 * d.normal,
                                        direction: -light.get_direction(element).normalize(),
                                    }, elements);
                                    color = color
                                        + match shadowed {
                                            false => element.get_color() * light.get_color() * intensity * reflected,
                                            true => element.get_color() * light.get_color() * 0.0 * reflected,
                                        };
                                }

                                temp.0 = color;
                                temp.1 = d.distance;
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
    pub fn is_shadowed(&self, ray: &Ray, elements: &[Element]) -> bool {
        let mut result = false;
        for element in elements {
            match element.intersect(ray) {
                Some(_) => result = true,
                _ => (),
            }
        }
        result
    }
}


