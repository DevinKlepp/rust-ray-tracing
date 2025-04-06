mod vec3;
mod colour;

use vec3::Vec3;
use colour::write_colour;

fn main() {
    // Image
    let image_width: u32 = 256;
    let image_height: u32 = 256;

    // Render (PPM File Format)
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in 0..image_height {
        eprint!("\r\x1B[KScanlines remaining {}", image_height - j - 1);
        for i in 0..image_width {
            let rgb: Vec3 = Vec3::new(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.0
            );
            write_colour(rgb);
        }
    }
    eprintln!("\nDone.");
}
