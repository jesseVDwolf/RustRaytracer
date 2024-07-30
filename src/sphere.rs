use serde::Deserialize;
use std::ops;

use crate::vec::Vec3;
use crate::point::Point3;
use crate::ray::Ray;
use crate::hit::Hit;


#[derive(Debug, Deserialize, Clone, Copy)]
pub struct Sphere {
    origin: Point3,
    radius: f64,
}

// TODO Create Vec4 later -> Homogeneous coordinates
// Non-Homogeneous -> Matrix * point + vec
// Homogeneous -> Matrix * Point

/*
* 
*/
impl Sphere {

    pub fn intersect(&self, r: &Ray) -> bool {
        // (C−P(t))⋅(C−P(t))=r2
        // (C−(Q+td))⋅(C−(Q+td))=r2
        // t2d⋅d−2td⋅(C−Q)+(C−Q)⋅(C−Q)−r2=0
        let cq = self.origin - r.origin;
        let a = r.direction.dot(&r.direction);
        let b = -2.0 * r.direction.dot(&cq);
        let c = cq.dot(&cq) - self.radius.powf(2.0);
        let discriminant = b.powf(2.0) - 4.0 * a * c;
        return discriminant >= 0.0;
    }
}


