use crate::Vec3;

pub fn write_colour(v: Vec3) {
    let r: u32 = (255.999 * v.x()) as u32;
    let g: u32 = (255.999 * v.y()) as u32;
    let b: u32 = (255.999 * v.z()) as u32;

    println!("{} {} {}", r, g, b);
}