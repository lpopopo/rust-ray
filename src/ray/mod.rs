use crate::color::vec3::Vec3;

pub struct Ray<'a> {
    orig: &'a Vec3,
    dir: &'a Vec3,
}

impl Ray<'_> {
    pub fn new<'a>(orig: &'a Vec3, dir: &'a Vec3) -> Ray<'a> {
        Ray { orig, dir }
    }

    pub fn orig(&self) -> &Vec3 {
        self.orig
    }

    pub fn dir(&self) -> &Vec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Vec3 {
        let ray = Vec3::new(
            self.orig.x() + t * self.dir.x(),
            self.orig.y() + t * self.dir.y(),
            self.orig.z() * t * self.dir.z(),
        );
        ray
    }

    pub fn ray_color(&self) -> Vec3 {
        let t = (self.dir.y() + 1.0) / 2.0;
        let res_ray = Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t;
        res_ray
    }
}
