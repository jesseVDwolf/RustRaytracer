use serde::Deserialize;

use crate::color::RGBAColor;
use crate::hit::Hit;
use crate::point::Point3;
use crate::ray::Ray;
use crate::vec::Vec3;
use crate::traits::Intersectable;

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Plane {
    origin: Point3,
    orient: Vec3,
    color: RGBAColor
}


impl Intersectable for Plane {

    fn intersect(&self, ray: &Ray) -> bool {
        let denominator = ray.direction.dot(&self.orient);
        denominator > 1e-6
    }

    fn intersect_hits(&self, ray: &Ray) -> Option<Vec<Hit>> {
        // t = -(Q - P) ⋅ N / d ⋅ N
        // t = (P - Q) ⋅ N / d ⋅ N
        let denominator = ray.direction.dot(&self.orient);
        
        // If the denominator is zero then no solutions
        // If the denominator is smaller then zero then t will be negative and
        // the hit will be behind us.
        // If the hit is only a bit bigger then zero then we might get
        // an infinite amount of solutions, which we don't want.
        if denominator > 1e-6 {
            let qp = self.origin - ray.origin;
            let numerator = qp.dot(&self.orient);
            let t = numerator / denominator;
            // TODO normal here is not aware of side of the plane it hits
            Some(vec![Hit{t, p: ray.at(t), normal: self.orient}])
        }
        else {
            None
        }
    }
}

