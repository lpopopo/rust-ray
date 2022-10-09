use crate::color::vec3::Vec3;
use crate::ray::Ray;
use crate::utils::degrees_to_radians;

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f64, aspect_ratio: f64) -> Camera {
        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);

        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();

        let viewport_height: f64 = h * 2.0;
        let viewport_width: f64 = aspect_ratio * viewport_height;

        let origin = lookfrom;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal * 0.5 - vertical * 0.5 - w;

        Camera {
            origin: origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let dir = self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin;
        Ray::new(self.origin, dir)
    }
}
