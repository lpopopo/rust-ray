use std::f64::consts::PI;

use rand;

pub fn random() -> f64 {
    rand::random::<f64>()
}

pub fn random_in(min: f64, max: f64) -> f64 {
    min + (max - min) * random()
}

pub fn clamp(num: f64, min: f64, max: f64) -> f64 {
    if num < min {
        min
    } else if num > max {
        max
    } else {
        num
    }
}

pub fn degrees_to_radians(degress: f64) -> f64 {
    degress * PI / 180.0
}
