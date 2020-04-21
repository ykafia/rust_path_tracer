use super::*;

#[derive(Copy, Clone,Debug)]
pub struct Triangle {
    pub coordinates: [Vector3<f32>; 3],
    pub color: Color,
    pub normal: Vector3<f32>,
    pub albedo: f32,
}

impl Triangle {
    pub fn new(coord: [Vector3<f32>; 3], col: Color, albedo: f32) -> Triangle {
        Triangle {
            coordinates: coord,
            color: col,
            normal: Triangle::calculate_normal(coord),
            albedo: albedo,
        }
    }
    pub fn new_defined() -> Triangle {
        let coord = [
            Vector3::new(0.0, 2.0, 0.0),
            Vector3::new(0.0, 5.0, 0.0),
            Vector3::new(5.0, 5.0, 5.0),
        ];
        Triangle {
            coordinates: coord,
            color: Colors::YELLOW.value(),
            normal: Triangle::calculate_normal(coord),
            albedo: 1.0,
        }
    }
    fn calculate_normal(coord: [Vector3<f32>; 3]) -> Vector3<f32> {
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
        (self.coordinates.to_vec().iter().sum::<Vector3<f32>>()) * (1.0/3.0)
    }
    fn intersect(&self, ray: &Ray) -> Option<PointInfo> {
        let normal = &self.normal;
        let denom = normal.dot(&ray.direction);
        if denom > 1e-6 {
            let d = self.normal.dot(&self.coordinates[0]);
            let t = -(self.normal.dot(&ray.origin) + d) / self.normal.dot(&ray.direction);
            let intersect = ray.origin + t * ray.direction;
            if check_inside_triangle(intersect,self.coordinates) {
                Some(
                    PointInfo {
                        distance : t,
                        intersection : intersect,
                        normal : -self.normal
                    }
                )
            } else {
                None
            }
        } else {
            None
        }

       
    }
    fn get_color(&self, intersection : Vector3<f32>) -> Color {
        self.color
    }
    fn get_albedo(&self) -> f32 {
        self.albedo
    }
    fn get_texcoord(&self, intersect : Vector3<f32>) -> TexCoord {
        // let triangle_area = compute_triangle_area(self.coordinates[1], self.coordinates[2], self.coordinates[0]);
        
        // This is the old way, calculating the triangle areas and all
        // let u = 
        //     //CAP area
        //     compute_triangle_area(self.coordinates[2], self.coordinates[0], intersect)/triangle_area;
        // let v = 
        //     // ABP area
        //     compute_triangle_area(self.coordinates[0], self.coordinates[1], intersect)/triangle_area;

        // let w = 
        //     //BCP area
        //     compute_triangle_area(self.coordinates[1], self.coordinates[2], intersect)/triangle_area;
        // 
        // But we're going to use a simpler math : 
        // Tri Area / Tri Area = Parallelogram Area * 0.5 / Parallelogram Area * 0.5
        //                     = Parallelogram Area / Parallelogram Area
        let parallelogram_area = |x : Vector3<f32>,y : Vector3<f32>, pivot : Vector3<f32>|  vector_length(x-pivot) * vector_length(y-pivot);
        
        let tri_p = parallelogram_area(self.coordinates[0],self.coordinates[1], self.coordinates[2]);
       
        //CAP
        let u = parallelogram_area(self.coordinates[2],self.coordinates[0],intersect)/tri_p;

        //ABP
        let v = parallelogram_area(self.coordinates[0],self.coordinates[1],intersect)/tri_p;

        //BCP
        let w = 1.0-u-v;
        
        TexCoord {
            x : u,
            y : v
        }
        

    }
}

fn compute_triangle_area( a : Vector3<f32>, b : Vector3<f32>, intersection : Vector3<f32>) -> f32 {
    let edge_1 = vector_length(b-intersection);
    let edge_2 = vector_length(a-intersection);
    edge_1*edge_2*0.5
}


fn check_inside_triangle(
    point: Vector3<f32>, 
    triangle: [Vector3<f32>; 3]) -> bool {
    same_side(point,triangle[0],triangle[1],triangle[2]) &&
        same_side(point,triangle[1],triangle[0],triangle[2]) &&
        same_side(point,triangle[2],triangle[0],triangle[1])     
}
fn same_side(p1 : Vector3<f32>,p2 : Vector3<f32>,a : Vector3<f32>,b : Vector3<f32>) -> bool {
    let cp1 = (b-a).cross(&(p1-a));
    let cp2 = (b-a).cross(&(p2-a));
    cp1.dot(&cp2) >= 0.0
}