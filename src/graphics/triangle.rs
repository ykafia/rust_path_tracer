use super::*;
use std::f32::consts::PI;

#[derive(Clone)]
pub struct Tri {
    pub coordinates: [Vector3<f32>; 3],
    pub normal : Vector3<f32>,
    pub material : Material
}

impl Tri {
    pub fn new(coord: [Vector3<f32>; 3], albedo: f32) -> Tri {
        let texture = image::io::Reader::open("textures/checker.png").unwrap().decode().unwrap();
        Tri {
            coordinates: coord,
            normal : Tri::calculate_normal(coord),
            material : Material {
                albedo : albedo,
                emissive : Surface::Texture(
                    texture
                ),
                reflectivity : None
            }
        }
    }
    pub fn new_defined() -> Tri {
        let coord = [
            Vector3::new(0.0, 2.0, 0.0),
            Vector3::new(0.0, 5.0, 0.0),
            Vector3::new(5.0, 5.0, 5.0),
        ];
        Tri {
            coordinates: coord,
            normal : Tri::calculate_normal(coord),
            material : Material {
                albedo : 1.0,
                emissive : Surface::Color(Colors::BLUE.value()),
                reflectivity : Some(1.0)
            }
        }
    }
    fn calculate_normal(coord: [Vector3<f32>; 3]) -> Vector3<f32> {
        let x = coord[1] - coord[0];
        let y = coord[2] - coord[0];
        x.cross(&y)
    }
}

impl Intersectable for Tri {
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
            if check_inside_Tri(intersect,self.coordinates) {
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
        self.material.emissive.color(
            self.get_texcoord(intersection)
        )
    }
    fn get_albedo(&self) -> f32 {
        self.material.albedo
    }
    fn get_texcoord(&self, intersect : Vector3<f32>) -> TexCoord {
        
        
        // This is the old way, calculating the Tri areas and all

        // let Tri_area = compute_Tri_area(self.coordinates[1], self.coordinates[2], self.coordinates[0]);
        // let u = 
        //     //CAP area
        //     compute_Tri_area(self.coordinates[2], self.coordinates[0], intersect)/Tri_area;
        // let v = 
        //     // ABP area
        //     compute_Tri_area(self.coordinates[0], self.coordinates[1], intersect)/Tri_area;

        // let w = 
        //     //BCP area
        //     compute_Tri_area(self.coordinates[1], self.coordinates[2], intersect)/Tri_area;
        
        // But we're going to use a simpler math : 
        // Tri Area / Tri Area = Parallelogram Area * 0.5 / Parallelogram Area * 0.5
        //                     = Parallelogram Area / Parallelogram Area
        let parallelogram_area = |x : Vector3<f32>,y : Vector3<f32>, pivot : Vector3<f32>|  vector_length(x-pivot) * vector_length(y-pivot);
        
        let tri_p = parallelogram_area(self.coordinates[0],self.coordinates[1], self.coordinates[2]);
       
        //CAP
        let u = parallelogram_area(self.coordinates[2],self.coordinates[0],intersect)/tri_p;

        //ABP
        let v = parallelogram_area(self.coordinates[0],self.coordinates[1],intersect)/tri_p;

        // BCP
        // let w = 1.0-u-v;
        // if v>3.0 {
        //     println!("[{} ; {} ; {}]",u,v,w);
        // }
        TexCoord {
            x : (u/PI) % 1.0,
            y : (v/PI) %1.0
        }
        

    }
    fn get_reflectivity(&self) -> Option<f32> {
        self.material.reflectivity
    }
}

impl ECSIntersectable for Tri {
    fn simple_intersect_ecs(&self, ray: &Ray, transform : &TransformComponent) -> bool {
        let normal = &self.normal;
        let denom = normal.dot(&ray.direction);
        if denom > 1e-6 {
            let p0l0 = transform.position - ray.origin;
            let t = p0l0.dot(&normal) / denom;
            t >= 0.0
        } else {
            false
        }
    }
    fn intersect_ecs(&self, ray: &Ray, transform : &TransformComponent) -> Option<PointInfo> {
        let tmp = self.coordinates.iter().map(|c| c+transform.position).collect::<Vec<Vector3<f32>>>();
        let coordinates : [Vector3<f32>; 3] = [tmp[0],tmp[1],tmp[2]];
        let normal = &self.normal;
        let denom = normal.dot(&ray.direction);
        if denom > 1e-6 {
            let d = self.normal.dot(&coordinates[0]);
            let t = -(self.normal.dot(&ray.origin) + d) / self.normal.dot(&ray.direction);
            let intersect = ray.origin + t * ray.direction;
            if check_inside_Tri(intersect,coordinates) {
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
    fn get_color_ecs(&self, intersection : Vector3<f32>, transform : &TransformComponent) -> Color {
        self.material.emissive.color(
            self.get_texcoord_ecs(intersection, transform)
        )
    }
    fn get_albedo_ecs(&self) -> f32 {
        self.material.albedo
    }
    fn get_texcoord_ecs(&self, intersect : Vector3<f32>, transform : &TransformComponent) -> TexCoord {
        
        
        // This is the old way, calculating the Tri areas and all

        // let Tri_area = compute_Tri_area(self.coordinates[1], self.coordinates[2], self.coordinates[0]);
        // let u = 
        //     //CAP area
        //     compute_Tri_area(self.coordinates[2], self.coordinates[0], intersect)/Tri_area;
        // let v = 
        //     // ABP area
        //     compute_Tri_area(self.coordinates[0], self.coordinates[1], intersect)/Tri_area;

        // let w = 
        //     //BCP area
        //     compute_Tri_area(self.coordinates[1], self.coordinates[2], intersect)/Tri_area;
        
        // But we're going to use a simpler math : 
        // Tri Area / Tri Area = Parallelogram Area * 0.5 / Parallelogram Area * 0.5
        //                     = Parallelogram Area / Parallelogram Area

        let tmp = self.coordinates.iter().map(|c| c+transform.position).collect::<Vec<Vector3<f32>>>();
        let coordinates : [Vector3<f32>; 3] = [tmp[0],tmp[1],tmp[2]];

        let parallelogram_area = |x : Vector3<f32>,y : Vector3<f32>, pivot : Vector3<f32>|  vector_length(x-pivot) * vector_length(y-pivot);
        
        let tri_p = parallelogram_area(coordinates[0],coordinates[1], coordinates[2]);
       
        //CAP
        let u = parallelogram_area(coordinates[2],coordinates[0],intersect)/tri_p;

        //ABP
        let v = parallelogram_area(coordinates[0],coordinates[1],intersect)/tri_p;

        // BCP
        // let w = 1.0-u-v;
        // if v>3.0 {
        //     println!("[{} ; {} ; {}]",u,v,w);
        // }
        TexCoord {
            x : (u/PI) % 1.0,
            y : (v/PI) %1.0
        }
        

    }
    fn get_reflectivity_ecs(&self) -> Option<f32> {
        self.material.reflectivity
    }
}

fn compute_Tri_area( a : Vector3<f32>, b : Vector3<f32>, intersection : Vector3<f32>) -> f32 {
    let edge_1 = vector_length(b-intersection);
    let edge_2 = vector_length(a-intersection);
    edge_1*edge_2*0.5
}


fn check_inside_Tri(
    point: Vector3<f32>, 
    Tri: [Vector3<f32>; 3]) -> bool {
    same_side(point,Tri[0],Tri[1],Tri[2]) &&
        same_side(point,Tri[1],Tri[0],Tri[2]) &&
        same_side(point,Tri[2],Tri[0],Tri[1])     
}
fn same_side(p1 : Vector3<f32>,p2 : Vector3<f32>,a : Vector3<f32>,b : Vector3<f32>) -> bool {
    let cp1 = (b-a).cross(&(p1-a));
    let cp2 = (b-a).cross(&(p2-a));
    cp1.dot(&cp2) >= 0.0
}