use super::*;
#[derive(Copy,Clone,Debug)]
pub enum Element {
    Sphere(Sphere),
    Plane(Plane),
    Triangle(Triangle)
}
impl Intersectable for Element {
    fn simple_intersect(&self, ray: &Ray) -> bool {
        match *self {
            Element::Sphere(ref s) => s.simple_intersect(ray),
            Element::Plane(ref p) => p.simple_intersect(ray),
            Element::Triangle(ref t) => t.simple_intersect(ray)
        }

    }
    fn intersect(&self, ray: &Ray) -> Option<PointInfo> {
        match *self {
            Element::Sphere(ref s) => s.intersect(ray),
            Element::Plane(ref p) => p.intersect(ray),
            Element::Triangle(ref t) => t.intersect(ray)
        }
    }
    fn get_color(&self) -> Color {
        match *self {
            Element::Sphere(ref s) => s.get_color(),
            Element::Plane(ref p) => p.get_color(),
            Element::Triangle(ref t) => t.get_color()
        }
    }
    fn get_albedo(&self) -> f32 {
        match *self {
            Element::Sphere(ref s) => s.get_albedo(),
            Element::Plane(ref p) => p.get_albedo(),
            Element::Triangle(ref t) => t.get_albedo()
        }
    }
    fn get_position(&self) -> Vector3<f32> {
        match *self {
            Element::Sphere(ref s) => s.get_position(),
            Element::Plane(ref p) => p.get_position(),
            Element::Triangle(ref t) => t.get_position()
        }
    }
}
