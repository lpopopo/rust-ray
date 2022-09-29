use crate::color::vec3::Vec3;
use crate::hittable::Hittable;
use crate::ray::Ray;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(e0: f64, e1: f64, e2: f64, radius: f64) -> Sphere {
        Sphere {
            center: Vec3::new(e0, e1, e2),
            radius,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut crate::hittable::HitRecord) -> bool {
        let oc = *ray.orig() - self.center;
        let a = ray.dir().length_squared();
        let half_b = ray.dir().dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }
        rec.set_t(root);
        rec.set_point(ray.at(rec.t()));
        rec.set_normal((rec.point - self.center) / self.radius);

        return true;
    }
}
