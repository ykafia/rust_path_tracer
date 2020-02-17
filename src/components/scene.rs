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
    #[allow(dead_code)]
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
                                    let intensity =
                                        d.normal.dot(&(-light.get_direction(element))).max(0.0)
                                            * light.get_intensity(d.intersection);
                                    let reflected = element.get_albedo() / PI;
                                    // let absorbed = Colors::WHITE.value() - element.get_color();
                                    // let final_color = light.color.clone() - absorbed;
                                    let shadowed = self.is_shadowed(
                                        &Ray {
                                            origin: d.intersection + 1e-5 * d.normal,
                                            direction: -light.get_direction(element).normalize(),
                                        },
                                        elements,
                                    );
                                    color = color
                                        + match shadowed {
                                            false => {
                                                element.get_color()
                                                    * light.get_color()
                                                    * intensity
                                                    * reflected
                                            }
                                            true => {
                                                element.get_color()
                                                    * light.get_color()
                                                    * 0.0
                                                    * reflected
                                            }
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
                temp = (lights[0].get_color() * lights[0].get_intensity(Vector3::new(0.0,0.0,0.0)) , std::f32::MAX);
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
    #[allow(dead_code)]
    pub fn fire_quarter_rays(
        &self,
        image: &mut DynamicImage,
        elements: &[Element],
        lights: &[Light],
        quarter: Quarter,
    ) -> DynamicImage {
        let mut temp: (Color, f32) = (Color::new(0.0, 0.0, 0.0, 0.0), std::f32::MAX);

        let (start_x,start_y,end_x,end_y) = match quarter {
            Quarter::TOPLEFT => (0,0,self.width/2,self.height/2),
            Quarter::TOPRIGHT => (self.width/2,0,self.width,self.height/2),
            Quarter::BOTTOMLEFT => (0,self.height/2,self.width/2,self.height),
            Quarter::BOTTOMRIGHT => (self.width/2,self.height/2,self.width,self.height)

        };

        for x in start_x..end_x {
            for y in start_y..end_y {
                for element in elements {
                    let ray = Ray::from_camera(x, y, self);
                    match element.intersect(&ray) {
                        Some(d) => {
                            if d.distance < temp.1 {
                                let mut color = Colors::BLACK.value();
                                for light in lights {
                                    let intensity =
                                        d.normal.dot(&(-light.get_direction(element))).max(0.0)
                                            * light.get_intensity(d.intersection);
                                    let reflected = element.get_albedo() / PI;
                                    // let absorbed = Colors::WHITE.value() - element.get_color();
                                    // let final_color = light.color.clone() - absorbed;
                                    let shadowed = self.is_shadowed(
                                        &Ray {
                                            origin: d.intersection + 1e-5 * d.normal,
                                            direction: -light.get_direction(element).normalize(),
                                        },
                                        elements,
                                    );
                                    color = color
                                        + match shadowed {
                                            false => {
                                                element.get_color()
                                                    * light.get_color()
                                                    * intensity
                                                    * reflected
                                            }
                                            true => {
                                                element.get_color()
                                                    * light.get_color()
                                                    * 0.0
                                                    * reflected
                                            }
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
}
#[allow(dead_code)]
pub enum Quarter {
    TOPLEFT,
    TOPRIGHT,
    BOTTOMLEFT,
    BOTTOMRIGHT,
}


pub struct ImgParts {
    tl : DynamicImage,
    tr : DynamicImage,
    bl : DynamicImage,
    br : DynamicImage
}

impl ImgParts {
    pub fn new(imgs : Vec<(Quarter, DynamicImage)>) -> ImgParts {
        let empty = DynamicImage::new_rgba8(1u32,1u32);
        let mut result = ImgParts {
            tl : empty.clone(),
            tr : empty.clone(),
            bl : empty.clone(),
            br : empty.clone()
        };
        for i in imgs {
            match i.0 {
                Quarter::TOPLEFT => result.tl = i.1,
                Quarter::TOPRIGHT => result.tr = i.1,
                Quarter::BOTTOMLEFT => result.bl = i.1,
                Quarter::BOTTOMRIGHT => result.br = i.1,
            }
        }
        result
    }

    pub fn get_width(&self) -> u32{
        self.tl.width()
    }
    pub fn get_height(&self) -> u32{
        self.tl.height()
    }
    pub fn get_result_image(&self) -> DynamicImage{
        let mut result = DynamicImage::new_rgba8(self.get_width(), self.get_height());
        for x in 0..self.get_width(){
            for y in 0..self.get_height() {
                result.put_pixel(x,y,
                    add_rgba(
                        self.tl.get_pixel(x, y),
                        self.tr.get_pixel(x, y),
                        self.bl.get_pixel(x, y),
                        self.br.get_pixel(x, y)
                    )
                );
            }
        }
        result
    }
    
}

pub fn add_rgba(pix1 : Rgba<u8>, pix2 : Rgba<u8>,pix3 : Rgba<u8>,pix4 : Rgba<u8>) -> Rgba<u8> {
    Rgba::<u8> {
        0 : [
            pix1[0] + pix2[0] + pix3[0] + pix4[0],
            pix1[1] + pix2[1] + pix3[1] + pix4[1],
            pix1[2] + pix2[2] + pix3[2] + pix4[2],
            pix1[3] + pix2[3] + pix3[3] + pix4[3]
        ]
    }
}