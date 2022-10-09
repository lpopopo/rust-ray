use crate::color::vec3::Vec3;
use crate::ray::Ray;
use crate::utils::degrees_to_radians;

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        //视角的单位向量
        let w = (lookfrom - lookat).unit_vector();
        //视角与vup形成平面的法向量的单位向量
        let u = vup.cross(w).unit_vector();
        //视角与u成平面的法向量的单位向量
        let v = w.cross(u);

        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();

        let viewport_height: f64 = h * 2.0;
        let viewport_width: f64 = aspect_ratio * viewport_height;

        let origin = lookfrom;
        let horizontal = viewport_width * u * focus_dist;
        let vertical = viewport_height * v * focus_dist;
        let lower_left_corner = origin - horizontal * 0.5 - vertical * 0.5 - w * focus_dist;

        let lens_radius = aperture / 2.0;

        Camera {
            origin: origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * rd.0 + self.v * rd.1;
        let dir =
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset;
        Ray::new(self.origin + offset, dir)
    }
}
