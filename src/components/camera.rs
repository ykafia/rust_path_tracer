use super::na::{Quaternion, Rotation3, Vector3,Matrix4};
use super::*;

#[derive(Clone)]
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
            fov: 70f32,
        }
    }
    pub fn change_rotation(&mut self, dir : Vector3<f32>) {
        self.rotation = Rotation3::face_towards(&dir,&Vector3::y());
    }
    pub fn yaw(&mut self, yaw: f32) {
        self.rotation = Rotation3::from_euler_angles(
            //roll pitch yaw
            self.rotation.euler_angles().0,
            self.rotation.euler_angles().1,
            self.rotation.euler_angles().2 + yaw,
        );
    }
    pub fn pitch(&mut self, pitch: f32) {
        self.rotation = Rotation3::from_euler_angles(
            self.rotation.euler_angles().0,
            self.rotation.euler_angles().1 + pitch,
            self.rotation.euler_angles().2,
        );
    }
    pub fn roll(&mut self, roll: f32) {
        self.rotation = Rotation3::from_euler_angles(
            self.rotation.euler_angles().0 + roll,
            self.rotation.euler_angles().1,
            self.rotation.euler_angles().2,
        );
    }
}
