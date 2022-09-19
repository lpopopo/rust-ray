use crate::color::vec3::Vec3;

pub struct Ray<'a> {
    orig: &'a Vec3,
    dir: &'a Vec3,
}

fn hit_sphere(center: &Vec3, radius: f64, r: &Ray) -> f64 {
    let oc = *r.orig() - *center;
    let a = r.dir().dot(*r.dir());
    let b = r.dir().dot(oc) * 2.0;
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    let res = if discriminant > 0.0 {
        1.0
    } else {
        -b - discriminant.sqrt() / (2.0 * a)
    };
    res
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
        let t = hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, &self);
        let res = if t > 0.0 {
            let n = (self.at(t) - Vec3::new(0.0, 0.0, -1.0)).unit_vec3();
            Vec3::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0) * 0.5
        } else {
            let unit_direction = (*self.dir).unit_vec3();
            let t = (unit_direction.y() + 1.0) / 2.0;
            let res_ray = Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t;
            res_ray
        };
        res
    }
}
