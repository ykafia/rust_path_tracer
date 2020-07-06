use super::na::Vector3;
use super::*;
use legion::prelude::*;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use std::f32::consts::PI;

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f32,
    pub camera: Camera,
    pub max_recursion: usize,
    pub world: World,
}

impl Scene {
    pub fn new(camera_pos: Vector3<f32>, target: Vector3<f32>, width: u32, height: u32) -> Scene {
        Scene {
            width: width,
            height: height,
            fov: 90.0,
            camera: Camera::new(
                // position
                camera_pos,
                // direction
                target.normalize(),
                width,
                height,
            ),
            max_recursion: 2,
            world: World::default(),
        }
    }

    pub fn compute_color(
        &self,
        ray: &Ray,
        closest_element: &Element,
        closest_point: PointInfo,
        elements: &[Element],
        color: Color,
        lights: &[Light],
        recursion: usize,
    ) -> Color {
        let computed_light_color = lights
            .iter()
            .map(|light| self.compute_shadowed(closest_element, light, closest_point, elements))
            .collect::<Vec<Color>>()
            .into_iter()
            .sum::<Color>();
        let mut new_color = color + computed_light_color;
        match (
            closest_element.get_reflectivity(),
            recursion < self.max_recursion,
        ) {
            (Some(r), true) => {
                // println!("Reflection again");
                let incident = closest_point.intersection - ray.origin;
                let new_ray = Ray {
                    origin: closest_point.intersection + 1e-4 * closest_point.normal,
                    direction: incident
                        - (2.0 * incident.dot(&closest_point.normal) * closest_point.normal),
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
                    .map(|(e, op)| RayInfo(e, op))
                    .collect::<Vec<RayInfo>>();
                intersects.sort();
                match intersects.first() {
                    Some(ri) => {
                        new_color = new_color
                            + self.compute_color(
                                &new_ray,
                                ri.0,
                                ri.1,
                                elements,
                                new_color.clone(),
                                lights,
                                recursion + 1,
                            ) * r
                    }
                    _ => (),
                }
            }
            _ => (),
        }
        new_color
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
    pub fn compute_shadowed(
        &self,
        element: &Element,
        light: &Light,
        pf: PointInfo,
        elements: &[Element],
    ) -> Color {
        let intensity = pf.normal.dot(&(-light.get_direction(&element))).max(0.0)
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
            false => element.get_color(pf.intersection) * light.get_color() * intensity * reflected,
        }
    }

    pub fn rayon_rays(
        &self,
        image: &mut DynamicImage,
        elements: &[Element],
        lights: &[Light],
    ) -> DynamicImage {
        let new_buffer = image
            .pixels()
            .collect::<Vec<(u32, u32, _)>>()
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
                    .map(|(e, op)| RayInfo(e, op))
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
                            0,
                        )
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
                        (Colors::SKYBLUE.value() * intensity).to_rgba()
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
    pub fn ecs_rays(&self, image: &mut DynamicImage) -> DynamicImage {
        let query_renderables = <(
            Read<RenderableComponent>,
            Read<TransformComponent>,
            Read<IDComponent>,
        )>::query();
        let query_lights = <(Read<LightComponent>, Read<TransformComponent>)>::query();

        let new_buffer = image
            .pixels()
            .collect::<Vec<(u32, u32, _)>>()
            .par_iter()
            .map(|(x, y, _)| {
                // check all intersect and compare the distances
                let ray = Ray::from_camera(*x, *y, self);
                let mut intersects = query_renderables
                    .into_iter(&self.world)
                    .map(|(el, tr, i)| ((&el, &tr), el.intersect_ecs(&ray, &tr)))
                    .filter(|(_, pi)| pi.is_some())
                    .map(|(el, pi)| (el, pi.unwrap()))
                    .map(|((e, tr), op)| ECSRayInfo::new((&e, &tr), op))
                    .collect::<Vec<ECSRayInfo>>();
                intersects.sort();
                match intersects.first() {
                    Some(v) => {
                        // for each element
                        let closest_element = v.e;
                        let closest_point = v.pi;
                        self.compute_color_ecs(&ray, 0, v.e, v.pi, Colors::BLACK.value(), 0)
                            .to_rgba()
                    }
                    None => {
                        let mut intensity = 1.0;
                        for (l, _) in query_lights.iter(&self.world) {
                            intensity = intensity
                                * match l.0 {
                                    Light::DirectionalLight(v) => v.intensity,
                                    _ => 1.0,
                                };
                        }
                        (Colors::SKYBLUE.value() * intensity).to_rgba()
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

    pub fn compute_color_ecs(
        &self,
        ray: &Ray,
        closest_id: usize,
        closest_element: (&RenderableComponent, &TransformComponent),
        closest_point: PointInfo,
        color: Color,
        recursion: usize,
    ) -> Color {
        let mut color_result = Colors::BLACK.value();
        let lights = <(Read<LightComponent>, Read<TransformComponent>)>::query();
        let elements = <(
            Read<RenderableComponent>,
            Read<TransformComponent>,
            Read<IDComponent>,
        )>::query();
        // let closest_element = elements.iter(&self.world).filter(|(_,_,i)| i.0 == closest_id).map(|(e,_,_)| e.0).collect::<Vec<Element>>().first().unwrap();
        let computed_light_color = lights
            .iter(&self.world)
            .map(|(light, transform)| {
                self.compute_shadowed_ecs(closest_element, &light, &transform, closest_point)
            })
            .collect::<Vec<Color>>()
            .into_iter()
            .sum::<Color>();
        let mut new_color = color + computed_light_color;
        match (
            closest_element.0.get_reflectivity(),
            recursion < self.max_recursion,
        ) {
            (Some(r), true) => {
                // println!("Reflection again");
                let incident = closest_point.intersection - ray.origin;
                let new_ray = Ray {
                    origin: closest_point.intersection + 1e-4 * closest_point.normal,
                    direction: incident
                        - (2.0 * incident.dot(&closest_point.normal) * closest_point.normal),
                };
                let mut intersects = elements
                    .iter(&self.world)
                    .map(|(el, tr, i)| ((el, tr), el.intersect_ecs(&ray, &tr)))
                    .filter(|(_, pi)| pi.is_some())
                    .map(|(el, pi)| (el, pi.unwrap()))
                    .map(|((e, tr), op)| ECSRayInfo::new((&e, &tr), op))
                    .collect::<Vec<ECSRayInfo>>();
                intersects.sort();

                match intersects.first() {
                    Some(ri) => {
                        new_color = new_color
                            + self.compute_color_ecs(
                                &new_ray,
                                0,
                                ri.e,
                                ri.pi,
                                new_color.clone(),
                                recursion + 1,
                            ) * r
                    }
                    _ => (),
                }
            }
            _ => (),
        }
        color_result = new_color;

        color_result
    }
    pub fn compute_shadowed_ecs(
        &self,
        element: (&RenderableComponent, &TransformComponent),
        light: &LightComponent,
        light_transform: &TransformComponent,
        pf: PointInfo,
    ) -> Color {
        let mut result = Colors::BLACK.value();
        let elements = <(
            Read<RenderableComponent>,
            Read<TransformComponent>,
            Read<IDComponent>,
        )>::query();

        let intensity = pf
            .normal
            .dot(&(-light.get_direction(&element.1, light_transform)))
            .max(0.0)
            * light.get_intensity(pf.intersection);
        let reflected = element.0.get_albedo_ecs() / PI;
        match self.is_shadowed_ecs(&Ray {
            origin: pf.intersection + 1e-4 * pf.normal,
            direction: -light.get_direction(element.1, light_transform).normalize(),
        }) {
            true => element.0.get_color(pf.intersection) * light.get_color() * 0.0 * reflected,
            false => element.0.get_color(pf.intersection) * light.get_color() * intensity * reflected,
        }
    }
    pub fn is_shadowed_ecs(&self, ray: &Ray) -> bool {
        let mut result = false;

        for (element, tr) in
            <(Read<RenderableComponent>, Read<TransformComponent>)>::query().iter(&self.world)
        {
            match element.intersect_ecs(ray, &tr) {
                Some(_) => result = true,
                _ => (),
            }
        }
        result
    }
}
