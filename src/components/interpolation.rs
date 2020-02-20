use super::*;
#[allow(dead_code)]
pub fn catmull(
    a: Vector3<f32>,
    b: Vector3<f32>,
    c: Vector3<f32>,
    d: Vector3<f32>,
    amount: f32,
) -> Vector3<f32> {
    let mut result = Vector3::new(0.0, 0.0, 0.0);
    let squared = amount * amount;
    let cubed = amount * squared;

    result.x = 0.5
        * ((((2.0 * b.x) + ((-a.x + c.x) * amount))
            + (((((2.0 * a.x) - (5.0 * b.x)) + (4.0 * c.x)) - d.x) * squared))
            + ((((-a.x + (3.0 * b.x)) - (3.0 * c.x)) + d.x) * cubed));

    result.y = 0.5
        * ((((2.0 * b.y) + ((-a.y + c.y) * amount))
            + (((((2.0 * a.y) - (5.0 * b.y)) + (4.0 * c.y)) - d.y) * squared))
            + ((((-a.y + (3.0 * b.y)) - (3.0 * c.y)) + d.y) * cubed));

    result.z = 0.5
        * ((((2.0 * b.z) + ((-a.z + c.z) * amount))
            + (((((2.0 * a.z) - (5.0 * b.z)) + (4.0 * c.z)) - d.z) * squared))
            + ((((-a.z + (3.0 * b.z)) - (3.0 * c.z)) + d.z) * cubed));

    result
}
