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
            let r: f64 = i as f64 / (image_width - 1) as f64;
            let g: f64 = j as f64 / (image_height - 1) as f64;
            let b: f64 = 0.0;

            let ir: u32 = (255.999 * r) as u32;
            let ig: u32 = (255.999 * g) as u32;
            let ib: u32 = (255.999 * b) as u32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
    eprintln!("\nDone.");
}
