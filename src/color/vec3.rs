#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    e: (f64, f64, f64),
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 { e: (e0, e1, e2) }
    }

    pub fn x(&self) -> f64 {
        self.e.0
    }

    pub fn y(&self) -> f64 {
        self.e.1
    }

    pub fn z(&self) -> f64 {
        self.e.2
    }

    pub fn dot(&self, other: Vec3) -> f64 {
        let d = self.x() * other.x() + self.y() * other.y() + self.z() * other.z();
        d
    }
}
/**
 * 实现Vec3 类型之间的加法
 */
impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: (
                self.x() + other.x(),
                self.y() + other.y(),
                self.z() + other.z(),
            ),
        }
    }
}
/**
 * 实现Vec3 类型之间减法
 */
impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: (
                self.x() - other.x(),
                self.y() - other.y(),
                self.z() - other.z(),
            ),
        }
    }
}
/**
 * 实现Vec3 类型之间乘法
 */
impl std::ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: (
                self.x() * other.x(),
                self.y() * other.y(),
                self.z() * other.z(),
            ),
        }
    }
}
impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: f64) -> Vec3 {
        Vec3 {
            e: (self.x() * other, self.y() * other, self.z() * other),
        }
    }
}
/**
 * 实现Vec3 类型之间除法
 */
impl std::ops::Div<Vec3> for Vec3 {
    type Output = Vec3;
    fn div(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: (
                self.x() / other.x(),
                self.y() / other.y(),
                self.z() / other.z(),
            ),
        }
    }
}
impl std::ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, other: f64) -> Vec3 {
        Vec3 {
            e: (self.x() / other, self.y() / other, self.z() / other),
        }
    }
}

// #[derive(Debug)]
// pub struct Adder(pub Vec3);

// impl std::ops::Add<Vec3> for Adder {
//     type Output = Vec3;
//     fn add(self, other: Vec3) -> Vec3 {
//         Vec3 {
//             e: (
//                 self.0.x() + other.x(),
//                 self.0.y() + other.y(),
//                 self.0.z() + other.z(),
//             ),
//         }
//     }
// }
// impl std::ops::Add<Adder> for Vec3 {
//     type Output = Vec3;
//     fn add(self, other: Adder) -> Vec3 {
//         Vec3 {
//             e: (
//                 self.x() + other.0.x(),
//                 self.y() + other.0.y(),
//                 self.z() + other.0.z(),
//             ),
//         }
//     }
// }
