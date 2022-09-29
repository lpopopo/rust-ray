mod image;
use image::Image;
use ray_tracing::color::vec3::Vec3;
use ray_tracing::color::{write_color, write_file_info};
use ray_tracing::hittable::{HitRecord, Hittable};
use ray_tracing::hittable_list::HittableList;
use ray_tracing::ray::Ray;
use ray_tracing::sphere::Sphere;

fn main() {
    //image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: f64 = 400.0;
    const IMAGE_HEIGHT: f64 = IMAGE_WIDTH / ASPECT_RATIO;
    let image: Image = Image::new(IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32);
    write_file_info(image.width(), image.height());

    //world
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(0.0, 0.0, -1.0, 0.5)));
    world.add(Box::new(Sphere::new(0.0, 1.0, 0.0, 0.1)));

    //Camera
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f64 = 1.0;
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

    //render
    for i in (0..image.height()).rev() {
        for j in 0..image.width() {
            let u: f64 = j as f64 / (IMAGE_WIDTH - 1.0);
            let v: f64 = i as f64 / (IMAGE_HEIGHT - 1.0);
            let dir_vec = lower_left_corner + horizontal * u + vertical * v;
            let r = Ray::new(&origin, &dir_vec);
            let pixel_color = r.ray_color(&world);
            write_color(&pixel_color);
        }
    }
}
