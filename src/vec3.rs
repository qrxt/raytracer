use std::ops;

#[derive(Debug, PartialEq)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new((x, y, z): (f64, f64, f64)) -> Vec3 {
        Self { x, y, z }
    }

    pub fn len(&self) -> f64 {
        let sum = self.x * self.x + self.y * self.y + self.z * self.z;

        sum.sqrt()
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

impl ops::Mul<f64> for Vec3 {
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
        self * converted
    }
}

impl ops::Div<i32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: i32) -> Self::Output {
        self * (1.0 / rhs as f64)
    }
}

//

#[test]
fn test_add_vectors() {
    let vec1 = Vec3::new((1.0, 3.0, -4.0));
    let vec2 = Vec3::new((2.0, 2.0, -2.0));

    let vec_result = Vec3::new((3.0, 5.0, -6.0));

    assert_eq!(vec1 + vec2, vec_result);
}

#[test]
fn test_scalar_mul() {
    let vec1 = Vec3::new((1.0, 2.0, 3.0));
    let scalar = 4;

    let vec_result = Vec3::new((4.0, 8.0, 12.0));

    assert_eq!(vec1 * scalar, vec_result);
}

#[test]
fn test_mul_vectors() {
    let vec1 = Vec3::new((1.0, 2.0, 3.0));
    let vec2 = Vec3::new((2.0, 3.0, 4.0));

    let vec_result = Vec3::new((2.0, 6.0, 12.0));

    assert_eq!(vec1 * vec2, vec_result);
}

#[test]
fn test_scalar_div() {
    let vec = Vec3::new((1.0, 2.0, 3.0));
    let scalar = 2;

    let vec_result = Vec3::new((0.5, 1.0, 1.5));

    assert_eq!(vec / scalar, vec_result);
}

#[test]
fn test_len() {
    let vec = Vec3::new((1.0, 2.0, 3.0));
    let len = format!("{:.4}", vec.len().to_string());

    assert_eq!(len, "3.74");
}
