use std::ops;

use crate::vec::Vec3;

pub type Point3 = Vec3;


// #[derive(Debug, Deserialize, Clone, Copy)]
// pub struct Point3 {
//     pub x: f64,
//     pub y: f64,
//     pub z: f64
// }

// impl ops::Add<Vec3> for Point3 {
//     type Output = Point3;

//     fn add(self, rhs: Vec3) -> Self::Output {
//         Self {
//             x: self.x + rhs.x,
//             y: self.y + rhs.y,
//             z: self.z + rhs.z
//         }
//     }
// }

// impl ops::Sub<Point3> for Point3 {
//     type Output = Point3;

//     fn sub(self, rhs: Point3) -> Self::Output {
//         Self {
//             x: self.x - rhs.x,
//             y: self.y - rhs.y,
//             z: self.z - rhs.z
//         }
//     }
// }
