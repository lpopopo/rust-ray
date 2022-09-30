pub mod vec3;
use vec3::Vec3;

pub fn write_file_info(width: u32, height: u32) {
    println!("P3\n{} {}\n255", width, height)
}

pub fn write_color(color: &Vec3, samples_per_pixel: f64) {
    let scale = 1.0 / samples_per_pixel;
    const COLOR_PIXEL: f64 = 256.0;
    println!(
        "{} {} {}",
        (COLOR_PIXEL * clamp(color.x() * scale, 0.0, 0.999).sqrt()) as u32,
        (COLOR_PIXEL * clamp(color.y() * scale, 0.0, 0.999).sqrt()) as u32,
        (COLOR_PIXEL * clamp(color.z() * scale, 0.0, 0.999).sqrt()) as u32
    )
}

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    };
    if x > max {
        return max;
    };
    return x;
}
