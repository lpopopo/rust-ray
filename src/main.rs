mod image;
use image::Image;
use rand;
use ray_tracing::camera::Camera;
use ray_tracing::color::vec3::Vec3;
use ray_tracing::color::{write_color, write_file_info};
use ray_tracing::hittable::{HitRecord, Hittable};
use ray_tracing::hittable_list::HittableList;
use ray_tracing::material::{Dielectric, Lambertian, Metal};
use ray_tracing::ray::Ray;
use ray_tracing::sphere::Sphere;
use ray_tracing::utils::{random, random_in};

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

fn random_scene() -> HittableList {
    let mut word = HittableList::new();

    let ground_material = Box::<Lambertian>::new(Lambertian::new(Vec3(0.5, 0.5, 0.5)));
    word.add(Box::<Sphere>::new(Sphere::new(
        0.0,
        -1000.0,
        0.0,
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random();
            let center = Vec3(a as f64 + 0.9 * random(), 0.3, b as f64 + 0.9 * random());

            if (center - Vec3(4.0, 0.2, 0.0)).len() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Vec3::random() * Vec3::random();
                    let sphere_material = Box::<Lambertian>::new(Lambertian::new(albedo));
                    word.add(Box::<Sphere>::new(Sphere::new(
                        center.0,
                        center.1,
                        center.2,
                        0.2,
                        sphere_material,
                    )))
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::random_in(0.5, 1.0);
                    let fuzz = random_in(0.0, 0.5);
                    let material = Box::<Metal>::new(Metal::new(albedo));
                    material.set_fuzz(fuzz);
                    word.add(Box::<Sphere>::new(Sphere::new(
                        center.0, center.1, center.2, 0.2, material,
                    )))
                } else {
                    let sphere_material = Box::<Dielectric>::new(Dielectric::new(1.5));
                    word.add(Box::<Sphere>::new(Sphere::new(
                        center.0,
                        center.1,
                        center.2,
                        0.2,
                        sphere_material,
                    )))
                };
            }
        }
    }

    let material1 = Box::<Dielectric>::new(Dielectric::new(1.5));
    let material2 = Box::<Lambertian>::new(Lambertian::new(Vec3(0.4, 0.2, 0.1)));
    let material3 = Box::<Metal>::new(Metal::new(Vec3(0.7, 0.6, 0.5)));

    word.add(Box::<Sphere>::new(Sphere::new(
        0.0, 1.0, 0.0, 1.0, material1,
    )));

    word.add(Box::<Sphere>::new(Sphere::new(
        -4.0, 1.0, 0.0, 1.0, material2,
    )));

    word.add(Box::<Sphere>::new(Sphere::new(
        4.0, 1.0, 0.0, 1.0, material3,
    )));
    word
}

fn main() {
    //image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: f64 = 384.0;
    const IMAGE_HEIGHT: f64 = IMAGE_WIDTH / ASPECT_RATIO;
    const MAX_DEPTH: u32 = 50;

    const SAMPLES_PER_PIXEL: u32 = 500;

    let image: Image = Image::new(IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32);
    write_file_info(image.width(), image.height());

    //world
    let mut world = random_scene();

    //Camera

    let lookfrom = Vec3(13.0, 2.0, 3.0);
    let lookat = Vec3(0.0, 0.0, 0.0);
    let vup = Vec3(0.0, 1.0, 0.0);
    let camera = Camera::new(lookfrom, lookat, vup, 20.0, ASPECT_RATIO, 0.1, 10.0);

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
