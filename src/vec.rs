use std::ops;

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vec3 {

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self{x, y, z}
    }

    /* Calculate the vector length (or magnitude) */
    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalise(&self) -> Self {
        let mag = self.magnitude();
        Self {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag
        }
    }

    pub fn dot(&self, v: &Self) -> f64 {
        (self.x * v.x) + (self.y * v.y) + (self.z * v.z)
    }
}


// Implement Vec3 * f64
impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

// Implement f64 * Vec3
impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        (1.0 / rhs) * self
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_vector_dot_same() {
        let v1 = Vec3{x: 0.0, y: 0.0, z: 1.0};
        let v2 = Vec3{x: 0.0, y: 0.0, z:1.0};

        assert!(v1.dot(&v2) == 1.0);
    }

    #[test]
    fn test_vector_dot_opposite() {
        let v1 = Vec3{x: 0.0, y: 0.0, z: 1.0};
        let v2 = Vec3{x: 0.0, y: 0.0, z:-1.0};

        assert!(v1.dot(&v2) == -1.0);
    }

    #[test]
    fn test_vector_dot_perpendicular() {
        let v1 = Vec3{x: 0.0, y: 0.0, z: 1.0};
        let v2 = Vec3{x: 0.0, y: 1.0, z:0.0};

        assert!(v1.dot(&v2) == 0.0);
    }
}
