use super::na::{Vector3,Rotation3};
use super::*;

pub struct Sphere {
    pub center : Vector3<f32>,
    pub color : Color,
    pub radius : f32
}

impl Sphere {
    pub fn new(x:f32,y:f32) -> Sphere {
        Sphere {
            center: Vector3::new(
                x,
                y,
                -5.0,
            ),
            radius: 1.0,
            color: Color {
                r: 0,
                g: 125,
                b: 225,
                a: 0
            },
        }
    }
}

impl Intersectable for Sphere{
    fn intersect(&self, ray : &Ray) -> bool{
        //Calculate the vector between ray and sphere
        let l = self.center - ray.origin;
        //Dot product to reveal the angle
        let adj2 = l.dot(&ray.direction);
        //Find the length-squared of the opposite side
        let d2 = l.dot(&l) - (adj2 * adj2);
        //If that length-squared is less than radius squared, the ray intersects the sphere
        d2 < (self.radius * self.radius)
    }
}