use super::na::{Rotation3, Vector3};

#[derive(Clone,Copy)]
pub struct Camera {
    pub position: Vector3<f32>,
    pub rotation: Rotation3<f32>,
    pub width: usize,
    pub height: usize,
    pub fov: f32,
}




impl Camera {
    pub fn new(position : Vector3<f32>, direction : Vector3<f32>) -> Camera {
        assert_ne!(Vector3::new(0.0,-1.0,0.0),direction.normalize());
        Camera {
            position: position,
            rotation: Rotation3::face_towards(&direction,&Vector3::y()),
            width: 600,
            height: 400,
            fov: 90f32,
        }
    }
    #[allow(dead_code)]
    pub fn change_rotation(&mut self, dir : Vector3<f32>) {
        self.rotation = Rotation3::face_towards(&dir,&Vector3::y());
    }
    
}
