use crate::color::vec3::Vec3;
use crate::ray::Ray;

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            point: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
        }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn set_point(&mut self, point: Vec3) {
        self.point = point
    }
    pub fn set_t(&mut self, t: f64) {
        self.t = t
    }
    pub fn set_normal(&mut self, normal: Vec3) {
        self.normal = normal
    }
    pub fn set_front_face(&mut self, ray: &Ray, outward_normal: Vec3) {
        let front_face = ray.dir().dot(outward_normal) < 0.0;
        self.front_face = front_face;
        self.normal = if front_face {
            outward_normal
        } else {
            outward_normal
        }
    }
}
pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
