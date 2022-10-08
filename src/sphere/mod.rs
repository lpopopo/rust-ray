use crate::color::vec3::Vec3;
use crate::hittable::Hittable;
use crate::material::Material;
use crate::ray::Ray;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub mat_ptr: Box<dyn Material + Send>,
}

impl Sphere {
    pub fn new(
        e0: f64,
        e1: f64,
        e2: f64,
        radius: f64,
        mat_ptr: Box<dyn Material + Send>,
    ) -> Sphere {
        Sphere {
            center: Vec3(e0, e1, e2),
            radius,
            mat_ptr,
        }
    }
}
fn in_min_max(t: f64, min: f64, max: f64) -> bool {
    if min > max {
        panic!(
            "Expected max > min, but received min is {}, max is {}",
            min, max
        );
    }
    t < max && t > min
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut crate::hittable::HitRecord) -> bool {
        let Sphere {
            center,
            radius,
            mat_ptr,
        } = self;
        let center = *center;
        let oc = ray.origin - center;
        let a = ray.direction.length_squared();
        let half_b = ray.direction.dot(oc);
        let c = oc.length_squared() - radius * radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let mut update_record = |temp: f64| {
                let point = ray.at(temp);
                let outward_normal = (point - center) / *radius;

                rec.set_t(temp);
                rec.set_point(point);
                rec.set_face_normal(&ray, outward_normal);
                rec.set_material(mat_ptr.box_clone());
            };

            let temp = (-half_b - root) / a;

            if in_min_max(temp, t_min, t_max) {
                update_record(temp);
                return true;
            }

            let temp = (-half_b + root) / a;

            if in_min_max(temp, t_min, t_max) {
                update_record(temp);
                return true;
            }
        }

        return false;
    }
}
