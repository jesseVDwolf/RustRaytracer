use crate::point::Point3;
use crate::vec::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Hit {
    pub t: f64,
    pub p: Point3,
    pub normal: Vec3,
}
