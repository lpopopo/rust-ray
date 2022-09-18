mod image;
use image::Image;
use ray_tracing::color::vec3::Vec3;
use ray_tracing::color::{write_color, write_file_info};

fn main() {
    let image: Image = Image::new(256, 256);
    write_file_info(image.width(), image.height());
    for i in 0..image.width() {
        for j in 0..image.height() {
            let color = Vec3::new(
                i as f64 / (image.width() - 1) as f64,
                j as f64 / (image.height() - 1) as f64,
                0.25,
            );
            write_color(&color);
        }
    }
}
