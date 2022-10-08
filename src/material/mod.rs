use crate::{color::vec3::Vec3, hittable::HitRecord, ray::Ray};
use std::fmt::Debug;

pub trait Material: Debug + Send + Sync {
    fn scatter(
        &mut self,
        ray_in: &Ray,
        record: &mut HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
    fn box_clone(&self) -> Box<dyn Material>;
}

#[derive(Debug)]
pub struct DefaultMaterial {}

impl DefaultMaterial {
    pub fn new() -> DefaultMaterial {
        DefaultMaterial {}
    }
}

impl Material for DefaultMaterial {
    fn scatter(
        &mut self,
        _ray_in: &Ray,
        _record: &mut HitRecord,
        _attenuation: &mut Vec3,
        _scattered: &mut Ray,
    ) -> bool {
        false
    }
    fn box_clone(&self) -> Box<dyn Material> {
        Box::new(DefaultMaterial::new())
    }
}

#[derive(Debug)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &mut self,
        _ray_in: &Ray,
        record: &mut HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let scatter_direction = record.normal + Vec3::random_unit_vector();
        *scattered = Ray::new(record.point, scatter_direction);
        *attenuation = self.albedo;
        true
    }
    fn box_clone(&self) -> Box<dyn Material> {
        Box::new(Lambertian::new(self.albedo))
    }
}

#[derive(Debug)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3) -> Metal {
        Metal { albedo, fuzz: 0.0 }
    }
    pub fn set_fuzz(&self, fuzz: f64) -> Self {
        let fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
        Metal {
            albedo: self.albedo,
            fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter(
        &mut self,
        ray_in: &Ray,
        record: &mut HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = Vec3::reflect(ray_in.direction.unit_vector(), record.normal);
        let reflected = reflected + Vec3::random_unit_vector() * self.fuzz;

        *scattered = Ray::new(record.point, reflected);
        *attenuation = self.albedo;
        scattered.direction.dot(record.normal) > 0.0
    }
    fn box_clone(&self) -> Box<dyn Material> {
        Box::new(Metal::new(self.albedo).set_fuzz(self.fuzz))
    }
}
