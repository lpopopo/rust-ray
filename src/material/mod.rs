use crate::utils::random;
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

#[derive(Debug)]
pub struct Dielectric {
    pub ir: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Dielectric {
        Dielectric {
            ir: index_of_refraction,
        }
    }
}
fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}
impl Material for Dielectric {
    fn scatter(
        &mut self,
        ray_in: &Ray,
        record: &mut HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Vec3::fill(1.0);
        let refraction_ratio = if record.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_direction = ray_in.direction.unit_vector();
        let cos_theta = 1.0_f64.min((-unit_direction).dot(record.normal));
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction = if cannot_refract || reflectance(cos_theta, refraction_ratio) > random() {
            Vec3::reflect(unit_direction, record.normal)
        } else {
            Vec3::refract(unit_direction, record.normal, refraction_ratio)
        };
        *scattered = Ray::new(record.point, direction);
        true
    }

    fn box_clone(&self) -> Box<dyn Material> {
        Box::new(Dielectric::new(self.ir))
    }
}
