use std::{
    fs::File,
    io::Write,
    time::{SystemTime, UNIX_EPOCH}
};

fn since_epoch() -> u128 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_millis(),
        Err(_) => 0
    }
}

fn main() -> std::io::Result<()> {
    const IMAGE_WIDTH : i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;

    let path = format!("image_{}.ppm", since_epoch());
    let mut image_file = File::create(&path)?;

    // PPM headers
    writeln!(image_file, "P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n")?;

    // pixels are written in rows from top to bottom
    for row in 0..IMAGE_HEIGHT {
        if row % 16 == 0 {
            println!("Scanlines remaining: {}", IMAGE_HEIGHT - row);
        }
        // rows are written from left to right
        for pixel in 0..IMAGE_HEIGHT {
            let r = (pixel as f64) / (IMAGE_WIDTH as f64 - 1.0);
            let g = (row as f64) / (IMAGE_HEIGHT as f64 - 1.0);
            let b = 0.0;

            let pixel_r = (255.999 * r) as i32;
            let pixel_g = (255.999 * g) as i32;
            let pixel_b = (255.999 * b) as i32;

            writeln!(image_file, "{} {} {}", pixel_r, pixel_g, pixel_b)?;
        }
    }
    println!("DONE! Saved to: {}", path);
    Ok(())
}
