use super::na::Vector3;
use super::*;
use std::f32::consts::PI;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

#[derive(Copy, Clone)]
pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f32,
    pub camera: Camera,
    pub max_recursion : usize
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
            max_recursion : 2,
        }
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
    pub fn compute_shadowed(&self, element : &Element,light : &Light, pf : PointInfo, elements : &[Element]) -> Color{
        let intensity = 
            pf
            .normal
            .dot(&(-light.get_direction(&element)))
            .max(0.0)
            * light.get_intensity(pf.intersection);
        let reflected = element.get_albedo() / PI;
        match self.is_shadowed(
            &Ray {
                origin: pf.intersection + 1e-4 * pf.normal,
                direction: -light.get_direction(&element).normalize(),
            },
            elements,
        ) {
            true => element.get_color(pf.intersection) * light.get_color() * 0.0 * reflected,
            false => element.get_color(pf.intersection) * light.get_color() * intensity * reflected
        }
    }
    
    pub fn rayon_rays(
        &self,
        image: &mut DynamicImage,
        elements: &[Element],
        lights: &[Light],
    ) -> DynamicImage {
        let new_buffer = 
            image
            .pixels()
            .collect::<Vec<(u32,u32,_)>>()
            .par_iter()
            .map(|(x, y, _)| {        
                // check all intersect and compare the distances
                let ray = Ray::from_camera(*x, *y, self);
                let temp = elements
                    .into_iter()
                    .map(|element| (element, element.intersect(&ray)))
                    .collect::<Vec<(&Element, Option<PointInfo>)>>();
                let mut temp2 = Vec::new();
                // Keep only the rays that hit
                for i in temp {
                    match i.1 {
                        Some(v) => temp2.push((i.0, v)),
                        None => (),
                    }
                }
                let mut intersects = temp2
                    .into_iter()
                    .map(|(e, op)| {RayInfo(e, op)})
                    .collect::<Vec<RayInfo>>();
                intersects.sort();
               
                match intersects.first() {
                    Some(v) => {
                        // for each element
                        let closest_element = v.0;
                        let closest_point = v.1;
                        lights
                            .iter()
                            .map(|light| {
                                self.compute_shadowed(
                                    &closest_element,
                                    light,
                                    closest_point,
                                    elements
                                )
                            })
                            .collect::<Vec<Color>>()
                            .into_iter()
                            .sum::<Color>()
                            .to_rgba()
                    }
                    None => {
                        let mut intensity = 1.0;
                        for l in lights {
                            intensity = intensity
                                * match l {
                                    Light::DirectionalLight(v) => v.intensity,
                                    _ => 1.0,
                                };
                        }
                        (Colors::SKYBLUE.value()*intensity).to_rgba()
                        
                    }
                }
                
            })
            .collect::<Vec<Rgba<u8>>>();

        let mut result =
            DynamicImage::new_rgba8(self.camera.width as u32, self.camera.height as u32);
        for y in 0..self.camera.height {
            for x in 0..self.camera.width {
                result.put_pixel(x as u32, y as u32, new_buffer[y * self.camera.width + x]);
            }
        }
        result
    }
}

