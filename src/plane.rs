use serde::Deserialize;

use crate::color::RGBAColor;
use crate::point::Point3;
use crate::vec::Vec3;

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Plane {
    origin: Point3,
    orient: Vec3,
    color: RGBAColor
}


