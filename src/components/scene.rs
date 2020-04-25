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

    pub fn compute_color(
        &self,
        ray : &Ray,
        closest_element : &Element,
        closest_point : PointInfo,
        elements : &[Element], 
        color : Color, 
        lights : &[Light],
        recursion : usize
    ) -> Color {
        
        let computed_light_color = 
            lights
            .iter()
            .map(|light| {
                self.compute_shadowed(
                    closest_element,
                    light,
                    closest_point,
                    elements
                )
            })
            .collect::<Vec<Color>>()
            .into_iter()
            .sum::<Color>();
        let mut new_color = color + computed_light_color;
        match (closest_element.get_reflectivity(),recursion<self.max_recursion) {
            (Some(r),true) =>{
                // println!("Reflection again");
                let incident = closest_point.intersection - ray.origin;
                let new_ray = Ray{
                    origin : closest_point.intersection + 1e-4 * closest_point.normal,
                    direction : incident - (2.0 * incident.dot(&closest_point.normal) * closest_point.normal), 
                };
                let temp = elements
                    .into_iter()
                    .map(|element| (element, element.intersect(&new_ray)))
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
                    Some(ri) => new_color = new_color + self.compute_color(&new_ray, ri.0, ri.1, elements, new_color.clone(), lights, recursion+1) * r,
                    _ =>(),
                }
                
            },
            _ => ()
        }
        new_color
        // let probable_elements_hit = 
        //     elements
        //     .iter()
        //     .map(
        //         |e| (e,e.intersect(ray))
        //     )
        //     .collect::<Vec<(&Element,Option<PointInfo>)>>();
        // let mut elements_hit = Vec::new();
        // for option in probable_elements_hit {
        //     match option.1 {
        //         Some(p) => elements_hit.push(RayInfo(option.0,p)),
        //         None => ()
        //     }
        // }
        // elements_hit.sort();
        // match elements_hit.first() {
        //     Some(ri) => {
        //         // TODO : Create a new ray from the hit point with a new direction
        //         // TODO : add the color to color value
        //         self.compute_color(ray, elements,color)
        //     },
        //     None => return color.clone()
        // }

        
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
                        self.compute_color(
                            &ray, 
                            closest_element,
                            closest_point,
                            elements, 
                            Colors::BLACK.value(), 
                            lights,
                            0
                        ).to_rgba()
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

