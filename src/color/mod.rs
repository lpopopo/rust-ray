pub mod vec3;
use vec3::Vec3;

pub fn write_file_info(width: u32, height: u32) {
    println!("P3\n{} {}\n255", width, height)
}

pub fn write_color(color: &Vec3) {
    const COLOR_PIXEL: f64 = 255.999;
    println!(
        "{} {} {}",
        (COLOR_PIXEL * color.x()) as u32,
        (COLOR_PIXEL * color.y()) as u32,
        (COLOR_PIXEL * color.z()) as u32
    )
}
