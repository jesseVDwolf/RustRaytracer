use crate::point::Point3;
use crate::vec::Vec3;


#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub direction: Vec3,
    pub origin: Point3
}

impl Ray {
    /* This constructor should be used. It makes sure the direction is always normalised. */
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self {
            origin,
            direction: direction.normalise()
        }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}
