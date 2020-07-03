use super::*;
pub enum Element {
    Sphere(Sphere),
    Plane(Plane),
    Triangle(Tri)
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
    fn get_color(&self, intersection : Vector3<f32>) -> Color {
        match *self {
            Element::Sphere(ref s) => s.get_color(intersection),
            Element::Plane(ref p) => p.get_color(intersection),
            Element::Triangle(ref t) => t.get_color(intersection)
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
    fn get_texcoord(&self, intersect : Vector3<f32>) -> TexCoord {
        match *self {
            Element::Sphere(ref s) => s.get_texcoord(intersect),
            Element::Plane(ref p) => p.get_texcoord(intersect),
            Element::Triangle(ref t) => t.get_texcoord(intersect)
        }
    }
    fn get_reflectivity(&self) -> Option<f32> {
        match *self {
            Element::Sphere(ref s) => s.get_reflectivity(),
            Element::Plane(ref p) => p.get_reflectivity(),
            Element::Triangle(ref t) => t.get_reflectivity()
        }
    
    }
}
