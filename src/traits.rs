use crate::{hit::Hit, ray::Ray};

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> bool;
    fn intersect_hits(&self, ray: &Ray) -> Option<Vec<Hit>>;
}
