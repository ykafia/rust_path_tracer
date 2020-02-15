use super::na::{Rotation3, Vector3};

#[derive(Clone,Copy)]
pub struct Camera {
    pub position: Vector3<f32>,
    pub rotation: Rotation3<f32>,
    pub width: u32,
    pub height: u32,
    pub fov: f32,
}




impl Camera {
    pub fn new(position : Vector3<f32>, direction : Vector3<f32>) -> Camera {
        Camera {
            position: position,
            rotation: Rotation3::face_towards(&direction,&Vector3::y()),
            width: 600,
            height: 400,
            fov: 90f32,
        }
    }
    pub fn change_rotation(&mut self, dir : Vector3<f32>) {
        self.rotation = Rotation3::face_towards(&dir,&Vector3::y());
    }
    
}
