use super::*;

#[derive(Copy,Clone)]
pub struct Triangle {
    pub coordinates : [Vector3<f32>;3],
    pub color : Color,
    pub normal : Vector3<f32>,
    pub albedo : f32
}

impl Triangle {
    pub fn new(coord : [Vector3<f32>;3], col : Color, albedo : f32) -> Triangle {
        Triangle {
            coordinates : coord,
            color : col,
            normal : Triangle::calculate_normal(coord),
            albedo : albedo
        }
    }
    pub fn new_defined() -> Triangle {
        let coord = [
            Vector3::new(0.0,0.0,0.0),
            Vector3::new(0.0,1.0,0.0),
            Vector3::new(3.0,1.0,3.0)
        ];
        Triangle {
            coordinates : coord,
            color : Colors::YELLOW.value(),
            normal : Triangle::calculate_normal(coord),
            albedo : 1.0
        }
    }
    fn calculate_normal(coord : [Vector3<f32>;3]) -> Vector3<f32> {
        let x = coord[1] - coord[0];
        let y = coord[2] - coord[0];
        x.cross(&y)
    }
}

impl Intersectable for Triangle {
    fn simple_intersect(&self, ray: &Ray) -> bool {
        let normal = &self.normal;
        let denom = normal.dot(&ray.direction);
        if denom > 1e-6 {
            let p0l0 = self.get_position() - ray.origin;
            let t = p0l0.dot(&normal) / denom;
            t >= 0.0
        } else {
            false
        }
    }
    fn get_position(&self) -> Vector3<f32> {
        self.coordinates
            .to_vec()
            .iter()
            .sum()
    }
    fn intersect(&self, ray: &Ray) -> Option<PointInfo> {
        
        let normal = &self.normal;
        let denom = normal.dot(&ray.direction);
        if denom > 1e-6 {
            let d = self.normal.dot(&self.coordinates[0]);
            let t = -(self.normal.dot(&ray.origin) + d) / self.normal.dot(&ray.direction);
            let intersection = ray.origin + t * ray.direction;
            let inside = {
                let edge0 = self.coordinates[1] - self.coordinates[0];
                let edge1 = self.coordinates[2] - self.coordinates[0];
                // let edge2 = self.coordinates[0] - self.coordinates[2];
                let c0 = intersection - self.coordinates[0];
                let c1 = intersection - self.coordinates[1];
                // let c2 = intersection - self.coordinates[2];
                if 
                    self.normal.dot(&(edge0.cross(&c0))) >0.0 &&
                    self.normal.dot(&(edge1.cross(&c1))) >0.0 {
                        true
                    }
                    else {
                        false
                    }
            };
            if t >= 0.0 && inside {
                return Some(
                    PointInfo {
                        distance : t,
                        normal : -self.normal,
                        intersection : intersection
                    }
                );
            }
        }
        None
    
        
        
    }
    fn get_color(&self) -> Color {
        self.color
    }
    fn get_albedo(&self) -> f32 {
        self.albedo
    }
}