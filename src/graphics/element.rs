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

impl ECSIntersectable for Element {
    fn simple_intersect_ecs(&self, ray: &Ray, transform : &TransformComponent) -> bool {
        match *self {
            Element::Sphere(ref s) => s.simple_intersect_ecs(ray, transform),
            Element::Plane(ref p) => p.simple_intersect_ecs(ray, transform),
            Element::Triangle(ref t) => t.simple_intersect_ecs(ray, transform)
        }

    }
    fn intersect_ecs(&self, ray: &Ray, transform : &TransformComponent) -> Option<PointInfo> {
        match *self {
            Element::Sphere(ref s) => s.intersect_ecs(ray, transform),
            Element::Plane(ref p) => p.intersect_ecs(ray, transform),
            Element::Triangle(ref t) => t.intersect_ecs(ray, transform)
        }
    }
    fn get_color_ecs(&self, intersection : Vector3<f32>, transform : &TransformComponent) -> Color {
        match *self {
            Element::Sphere(ref s) => s.get_color_ecs(intersection, transform),
            Element::Plane(ref p) => p.get_color_ecs(intersection, transform),
            Element::Triangle(ref t) => t.get_color_ecs(intersection, transform)
        }
    }
    fn get_albedo_ecs(&self) -> f32 {
        match *self {
            Element::Sphere(ref s) => s.get_albedo(),
            Element::Plane(ref p) => p.get_albedo(),
            Element::Triangle(ref t) => t.get_albedo()
        }
    }
    fn get_texcoord_ecs(&self, intersect : Vector3<f32>, transform : &TransformComponent) -> TexCoord {
        match *self {
            Element::Sphere(ref s) => s.get_texcoord_ecs(intersect, transform),
            Element::Plane(ref p) => p.get_texcoord_ecs(intersect, transform),
            Element::Triangle(ref t) => t.get_texcoord_ecs(intersect, transform)
        }
    }
    fn get_reflectivity_ecs(&self) -> Option<f32> {
        match *self {
            Element::Sphere(ref s) => s.get_reflectivity(),
            Element::Plane(ref p) => p.get_reflectivity(),
            Element::Triangle(ref t) => t.get_reflectivity()
        }
    
    }
}
