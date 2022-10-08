mod image;
use image::Image;
use rand;
use ray_tracing::camera::Camera;
use ray_tracing::color::vec3::Vec3;
use ray_tracing::color::{write_color, write_file_info};
use ray_tracing::hittable::{HitRecord, Hittable};
use ray_tracing::hittable_list::HittableList;
use ray_tracing::material::{Lambertian, Metal};
use ray_tracing::ray::Ray;
use ray_tracing::sphere::Sphere;

fn ray_color<W>(ray: &Ray, world: &W, depth: u32) -> Vec3
where
    W: Hittable,
{
    if depth <= 0 {
        return Vec3::fill(0.0);
    }

    let mut record = HitRecord::new();

    if world.hit(&ray, 0.001, f64::INFINITY, &mut record) {
        let mut scattered = Ray::new(Vec3::fill(0.0), Vec3::fill(0.0));
        let mut attenuation = Vec3::fill(0.0);
        let mut material = record.material.box_clone();

        if material.scatter(&ray, &mut record, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        } else {
            return Vec3::fill(0.0);
        }
    }

    let unit_direction = Vec3::unit_vector(&ray.direction);
    let t = 0.5 * (unit_direction.y() + 1.0);

    let from = Vec3(1.0, 1.0, 1.0);
    let to = Vec3(0.5, 0.7, 1.0);

    Vec3::lerp(t, from, to)
}

fn main() {
    //image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: f64 = 400.0;
    const IMAGE_HEIGHT: f64 = IMAGE_WIDTH / ASPECT_RATIO;
    const MAX_DEPTH: u32 = 50;

    const SAMPLES_PER_PIXEL: u32 = 100;

    let image: Image = Image::new(IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32);
    write_file_info(image.width(), image.height());

    //world
    let mut world = HittableList::new();
    let material_ground = Box::<Lambertian>::new(Lambertian::new(Vec3(0.8, 0.8, 0.0)));
    let material_center = Box::<Lambertian>::new(Lambertian::new(Vec3(0.7, 0.3, 0.3)));
    let material_left = Box::<Metal>::new(Metal::new(Vec3(0.8, 0.8, 0.8)));
    let material_right = Box::<Metal>::new(Metal::new(Vec3(0.8, 0.6, 0.2)));
    material_left.set_fuzz(0.3);
    material_right.set_fuzz(1.0);

    world.add(Box::new(Sphere::new(
        0.0,
        -100.5,
        -1.0,
        100.0,
        material_ground,
    )));
    world.add(Box::new(Sphere::new(0.0, 0.0, -1.0, 0.5, material_center)));
    world.add(Box::new(Sphere::new(-1.0, 0.0, -1.0, 0.5, material_left)));
    world.add(Box::new(Sphere::new(1.0, 0.0, -1.0, 0.5, material_right)));

    //Camera
    let camera = Camera::new();

    //render
    for i in (0..image.height()).rev() {
        for j in 0..image.width() {
            let mut pixel_color = Vec3(0.0, 0.0, 0.0);
            for _s in 0..SAMPLES_PER_PIXEL {
                let u: f64 = (j as f64 + rand::random::<f64>()) / (IMAGE_WIDTH - 1.0);
                let v: f64 = (i as f64 + rand::random::<f64>()) / (IMAGE_HEIGHT - 1.0);

                let ray = camera.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&ray, &mut world, MAX_DEPTH)
            }
            write_color(&pixel_color, SAMPLES_PER_PIXEL as f64);
        }
    }
}
