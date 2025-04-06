mod vec3;
mod colour;
mod ray;

use vec3::Vec3;
use ray::Ray;
use colour::write_colour;
use colour::ray_colour;

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

    // Camera / Eye
    const FOCAL_LENGTH: f64 = 1.0;
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    const ORIGIN: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };

    // Viewport Edges
    let horizontal_edge: Vec3 = Vec3 {
        x: VIEWPORT_WIDTH,
        y: 0.0,
        z: 0.0,
    };
    let vertical_edge: Vec3 = Vec3 {
        x: 0.0,
        y: -VIEWPORT_HEIGHT,
        z: 0.0,
    };

    // Viewport Depth
    let depth: Vec3 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: FOCAL_LENGTH,
    };

    // Delta Vectors
    let delta_u = horizontal_edge / (IMAGE_WIDTH as f64);
    let delta_v = vertical_edge / (IMAGE_HEIGHT as f64);

    // Upper Left Pixel Calculation
    let upper_left_corner: Vec3 = ORIGIN - depth - (horizontal_edge / 2.0) - (vertical_edge / 2.0);
    let upper_left_pixel: Vec3 = upper_left_corner + ((delta_u + delta_v) * 0.5);


    // Render (PPM File Format)
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in 0..IMAGE_HEIGHT {
        eprint!("\r\x1B[KScanlines remaining {}", IMAGE_HEIGHT - j - 1);
        for i in 0..IMAGE_WIDTH {
            let pixel_center: Vec3 = upper_left_pixel + (delta_u * (i as f64)) + (delta_v * (j as f64));
            let direction: Vec3 = pixel_center - ORIGIN;

            let r: Ray = Ray::new(ORIGIN, direction);
            let colour: Vec3 = ray_colour(r);
            write_colour(colour);
        }
    }
    eprintln!("\nDone.");
}
