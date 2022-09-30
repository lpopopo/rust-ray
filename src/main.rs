mod image;
use image::Image;
use rand::prelude::*;
use ray_tracing::camera::Camera;
use ray_tracing::color::vec3::Vec3;
use ray_tracing::color::{write_color, write_file_info};
use ray_tracing::hittable_list::HittableList;
use ray_tracing::ray::Ray;
use ray_tracing::sphere::Sphere;

fn main() {
    //random
    let mut rng = rand::thread_rng();

    //image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: f64 = 400.0;
    const IMAGE_HEIGHT: f64 = IMAGE_WIDTH / ASPECT_RATIO;

    const SAMPLES_PER_PIXEL: u32 = 100;

    let image: Image = Image::new(IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32);
    write_file_info(image.width(), image.height());

    //world
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(0.0, 0.0, -1.0, 0.5)));
    world.add(Box::new(Sphere::new(0.0, 1.0, 0.0, 0.1)));

    //Camera
    let mut camera = Camera::new();

    //render
    for i in (0..image.height()).rev() {
        for j in 0..image.width() {
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
            for s in 0..SAMPLES_PER_PIXEL {
                let u: f64 = (j as f64 + rng.gen::<f64>()) / (IMAGE_WIDTH - 1.0);
                let v: f64 = (i as f64 + rng.gen::<f64>()) / (IMAGE_HEIGHT - 1.0);

                let ray = camera.get_ray(u, v);
                pixel_color = pixel_color + ray.ray_color(&mut world)
            }
            write_color(&pixel_color, SAMPLES_PER_PIXEL as f64);
        }
    }
}
