use super::na::Vector3;
use super::*;

pub struct Ray {
    pub origin: Vector3<f32>,
    pub direction: Vector3<f32>,
}

impl Ray {
    /// X and Y are the coordinates of the pixels
    /// Let's switch to another type of problem with a camera and some math.
    /// First the fov will be the actual distance between the center of the image and the camera
    /// to get the angle : camera point - (center point + (get centered index))
    #[allow(dead_code)]
    pub fn new(x: u32, y: u32, scene: &Scene) -> Ray {
        assert!(scene.width > scene.height);
        let fov_adjustment = (scene.fov.to_radians() / 2.0).tan();
        let aspect_ratio = (scene.width as f32) / (scene.height as f32);
        let dir_x = ((((x as f32 + 0.5) / scene.width as f32) * 2.0 - 1.0) * aspect_ratio) * fov_adjustment;
        let dir_y = 1.0 - ((y as f32 + 0.5) / scene.height as f32) * 2.0;
        Ray {
            origin: Vector3::new(0f32, 0f32, 0f32),
            direction: Vector3::new(dir_x, dir_y, -1f32).normalize(),
        }
    }
    pub fn from_camera(x: u32, y: u32, scene: &Scene) -> Ray {
        assert!(scene.width > scene.height);
        let fov_adjustment = (scene.camera.fov.to_radians() / 2.0).tan();
        let aspect_ratio = (scene.width as f32) / (scene.height as f32);
        let dir_x = ((((x as f32 + 0.5) / scene.width as f32) * 2.0 - 1.0) * aspect_ratio) * fov_adjustment;
        let dir_y = 1.0 - ((y as f32 + 0.5) / scene.height as f32) * 2.0;
        Ray {
            origin: scene.camera.position,
            direction: (scene.camera.rotation * Vector3::new(dir_x, dir_y, 1f32).normalize()).normalize(),
        }
    }
}
