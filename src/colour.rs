use crate::Vec3;
use crate::Ray;

pub fn write_colour(v: Vec3) {
    let r: u32 = (255.999 * v.x) as u32;
    let g: u32 = (255.999 * v.y) as u32;
    let b: u32 = (255.999 * v.z) as u32;

    println!("{} {} {}", r, g, b);
}

pub fn ray_colour(r: Ray) -> Vec3 {
    let unit_direction: Vec3 = r.direction.unit_vector();
    let a: f64 = 0.5 * (unit_direction.y + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a
}