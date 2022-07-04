use std::ops;

#[derive(Debug, PartialEq, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[macro_export]
macro_rules! vec3 {
    ($x:expr, $y:expr, $z:expr) => {{
        let x_converted = $x as f64;
        let y_converted = $y as f64;
        let z_converted = $z as f64;

        Vec3 {
            x: x_converted,
            y: y_converted,
            z: z_converted,
        }
    }};
}

#[test]
fn test_vec3_macro() {
    assert_eq!(vec3!(0, 1, 2), Vec3::new((0.0, 1.0, 2.0)));
    assert_eq!(vec3!(4.0, 11.0, -2.0), Vec3::new((4.0, 11.0, -2.0)));
    assert_eq!(vec3!(-1, -1, -1), Vec3::new((-1.0, -1.0, -1.0)));
}

impl Vec3 {
    pub fn new((x, y, z): (f64, f64, f64)) -> Vec3 {
        Self { x, y, z }
    }

    pub fn len(&self) -> f64 {
        let sum = self.x * self.x + self.y * self.y + self.z * self.z;

        sum.sqrt()
    }

    pub fn unit(&self) -> Vec3 {
        self / self.len()
    }

    pub fn r(&self) -> f64 {
        self.x
    }

    pub fn g(&self) -> f64 {
        self.y
    }

    pub fn b(&self) -> f64 {
        self.z
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<'a> ops::Add<Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl<'a> ops::Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Mul<i32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: i32) -> Self::Output {
        let converted = rhs as f64;
        &self * converted
    }
}

impl ops::Div<i32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: i32) -> Self::Output {
        &self * (1.0 / rhs as f64)
    }
}

impl<'a> ops::Div<f64> for &'a Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

pub fn dot(vec1: Vec3, vec2: Vec3) -> f64 {
    (vec1.x * vec2.x) + (vec1.y * vec2.y) + (vec1.z * vec2.z)
}

pub fn cross(vec1: Vec3, vec2: Vec3) -> Vec3 {
    Vec3 {
        x: vec1.y * vec2.z - vec1.z * vec2.y,
        y: vec1.z * vec2.x - vec1.x * vec2.z,
        z: vec1.x * vec2.y - vec1.y * vec2.x,
    }
}

//

#[test]
fn test_add_vectors() {
    let vec1 = vec3!(1.0, 3.0, -4.0);
    let vec2 = vec3!(2.0, 2.0, -2.0);

    let vec_expected = vec3!(3.0, 5.0, -6.0);

    assert_eq!(vec1 + vec2, vec_expected);
}

#[test]
fn test_scalar_mul() {
    let vec1 = vec3!(1.0, 2.0, 3.0);
    let scalar = 4;

    let vec_expected = vec3!(4.0, 8.0, 12.0);

    assert_eq!(vec1 * scalar, vec_expected);
}

#[test]
fn test_mul_vectors() {
    let vec1 = vec3!(1.0, 2.0, 3.0);
    let vec2 = vec3!(2.0, 3.0, 4.0);

    let vec_expected = vec3!(2.0, 6.0, 12.0);

    assert_eq!(vec1 * vec2, vec_expected);
}

// #[test]
// fn test_scalar_div() {
//     let vec = vec3!(8, 6, 7);

//     let vec_expected = vec3!(4, 3, 3.5);

//     assert_eq!(vec / 2, vec_expected);
// }

#[test]
fn test_scalar_div() {
    let vec = vec3!(1.0, 2.0, 3.0);
    let scalar = 2;

    let vec_expected = vec3!(0.5, 1.0, 1.5);

    assert_eq!(vec / scalar, vec_expected);
}

#[test]
fn test_len() {
    let vec = vec3!(1.0, 2.0, 3.0);
    let len = format!("{:.4}", vec.len().to_string());

    assert_eq!(len, "3.74");
}

#[test]
fn test_dot() {
    let vec1 = vec3!(1.0, 2.0, 3.0);
    let vec2 = vec3!(3.0, 2.0, 1.0);

    assert_eq!(dot(vec1, vec2), 10.0);
}

#[test]
fn test_cross() {
    let vec1 = vec3!(1.0, 2.0, 3.0);
    let vec2 = vec3!(3.0, 2.0, 1.0);

    let result_vec = vec3!(-4.0, 8.0, -4.0);

    assert_eq!(cross(vec1, vec2), result_vec);
}

#[test]
fn test_unit() {
    let vec = vec3!(1.0, 2.0, 3.0);

    let vec_unit = vec.unit();
    let vec_expected = vec3!(0.26, 0.53, 0.80);

    let x = f64::trunc(&vec_unit.x * 100.0) / 100.0;
    let y = f64::trunc(&vec_unit.y * 100.0) / 100.0;
    let z = f64::trunc(&vec_unit.z * 100.0) / 100.0;

    assert_eq!(&x, &vec_expected.x);
    assert_eq!(&y, &vec_expected.y);
    assert_eq!(&z, &vec_expected.z);
}
