use super::na::Vector3;
use super::*;

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f32,
    pub camera: Camera,
    pub elements: Vec<Box<dyn Intersectable>>,
    pub directional_light : DirectionalLight
}

impl Scene {
    pub fn new(cameraPos : Vector3<f32>) -> Scene {
        let mut result = Scene {
            width: 600,
            height: 400,
            fov: 90.0,
            camera: Camera::new(
                // position
                cameraPos,
                // direction
                Vector3::new(0f32, 0f32, 0f32).normalize()
            ),
            elements: vec![
                Box::new(Sphere::new(0f32, 0f32, -3f32,Colors::BLUE,1f32)),
                Box::new(Sphere::new(0f32, 1f32, -4f32,Colors::RED,1.0)),
                Box::new(Sphere::new(1f32, 1f32, -1f32,Colors::GREEN,0.8)),
                Box::new(Plane::new()),
            ],
            directional_light : DirectionalLight {
                direction : Vector3::new(0f32,-1f32,-1f32),
                color : Colors::WHITE.value(),
                intensity : 0.5
            }
            
        };
        result.camera.change_rotation(
            result.elements.first().expect("a vector").get_position() - cameraPos
        );
        result
    }
    pub fn fire_rays(&self, image: &mut DynamicImage) -> DynamicImage {
        let mut temp: (Color, f32) = (Color::new(0, 0, 0, 0), std::f32::MAX);
        for x in 0..self.width {
            for y in 0..self.height {
                for element in &self.elements {
                    let ray = Ray::from_camera(x, y, self);
                    match element.intersect(&ray) {
                        Some(d) => {
                            if d.distance < temp.1 {
                                let intensity = (d.normal.dot(&self.directional_light.direction)) * &self.directional_light.intensity;
                                let reflected = element.get_albedo(); // /std::f32::consts::PI;
                                //println!("intensity {} reflected {}",intensity,reflected);
                                temp.1 = d.distance;
                                
                                temp.0 = element.get_color() * intensity * reflected ;//* self.directional_light.color.clone();
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
}

pub struct DirectionalLight {
    direction : Vector3<f32>,
    color : Color,
    intensity : f32
}
