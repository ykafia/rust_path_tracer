use super::*;

pub use self::camera::*;
pub use self::plane::*;
pub use self::ray::*;
#[allow(dead_code)]
pub use self::scene::*;
pub use self::sphere::*;
pub use self::utils::*;
pub use self::interpolation::*;
pub use self::lights::*;
pub use self::triangle::*;


mod camera;
mod plane;
mod ray;
mod scene;
mod sphere;
mod utils;
mod interpolation;
mod lights;
mod triangle;