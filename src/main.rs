fn main() {
    const IMAGE_WIDTH : i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;

    // PPM headers
    print!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");

    // pixels are written in rows from top to bottom
    for row in 0..IMAGE_HEIGHT {
        // rows are written from left to right
        for pixel in 0..IMAGE_HEIGHT {
            let r = (pixel as f64) / (IMAGE_WIDTH as f64 - 1.0);
            let g = (row as f64) / (IMAGE_HEIGHT as f64 - 1.0);
            let b = 0.0;

            let pixel_r = (255.999 * r) as i32;
            let pixel_g = (255.999 * g) as i32;
            let pixel_b = (255.999 * b) as i32;
            println!("{pixel_r} {pixel_g} {pixel_b}");
        }
    }
}
