use serde::Deserialize;

use crate::point::Point3;
use crate::ray::Ray;
use crate::hit::Hit;
use crate::vec::Vec3;


#[derive(Debug, Deserialize, Clone, Copy)]
pub struct Sphere {
    pub origin: Point3,
    pub radius: f64,
}

// TODO Create Vec4 later -> Homogeneous coordinates
// Non-Homogeneous -> Matrix * point + vec
// Homogeneous -> Matrix * Point

struct Quadratic {
    discriminant: f64,
    solutions: Option<Vec<f64>>
}

impl Quadratic {
    pub fn new(a: f64, b: f64, c: f64) -> Self {
        let discriminant = b.powf(2.0) - 4.0 * a * c;
        match discriminant {
            _ if discriminant < 0.0 => {
                Self { discriminant, solutions: None }
            },
            _ if discriminant == 0.0 => {
                let t = -b / (2.0 * a);
                Self { discriminant, solutions: Some(vec![t]) }
            },
            _ => {
                let t0 = -b + (b.powf(2.0) - 4.0 * a * c).sqrt() / (2.0 * a);
                let t1 = b + (b.powf(2.0) - 4.0 * a * c).sqrt() / (2.0 * a);
                Self {
                    discriminant,
                    solutions: Some(vec![t0.max(t1), t0.min(t1)])
                }
            }
        }
    }

    pub fn new_with_discriminant(a: f64, h: f64, discriminant: f64) -> Self {
         match discriminant {
            _ if discriminant < 0.0 => {
                Self { discriminant, solutions: None }
            },
            _ if discriminant == 0.0 => {
                let t = h / a;
                Self { discriminant, solutions: Some(vec![t]) }
            },
            _ => {
                let t0 = h + discriminant.sqrt() / a;
                let t1 = h + discriminant.sqrt() / a;
                Self {
                    discriminant,
                    solutions: Some(vec![t0.max(t1), t0.min(t1)])
                }
            }
        }
    }
}

/*
* 
*/
impl Sphere {
    
    pub fn get_quadratic(&self, r: &Ray) -> Quadratic {
        // (C−P(t))⋅(C−P(t))=r2
        // (C−(Q+td))⋅(C−(Q+td))=r2
        // t2d⋅d−2td⋅(C−Q)+(C−Q)⋅(C−Q)−r2=0
        // a -> d⋅d
        // b -> −2d⋅(C−Q)
        // c -> (C−Q)⋅(C−Q)−r2
        let cq = self.origin - r.origin;
        let a = r.direction.dot(&r.direction);
        let b = -2.0 * r.direction.dot(&cq);
        let c = cq.dot(&cq) - self.radius.powf(2.0);
        Quadratic::new(a, b, c)
    }

    pub fn get_quadratic_v2(&self, r: &Ray) -> Quadratic {
        // 6.2 Simplifying the Ray-Sphere Intersection Code
        // https://raytracing.github.io/books/RayTracingInOneWeekend.html#rays,asimplecamera,andbackground
        let cq = self.origin - r.origin;
        let a = r.direction.magnitude().powf(2.0);
        let h = r.direction.dot(&cq);
        let c = cq.magnitude().powf(2.0) - self.radius.powf(2.0);
        let discriminant = h * h - a * c;
        Quadratic::new_with_discriminant(a, h, discriminant)
    }

    pub fn intersect(&self, r: &Ray) -> bool {
        self.get_quadratic(r).discriminant >= 0.0
    }

    pub fn intersect_hits(&self, r: &Ray) -> Option<Vec<Hit>> {
        // Follow the ray and find the intersections with the sphere.
        // There are some options here.
        // 1. The ray is outside of the sphere and hits the sphere two times.
        //    In this case the vector will contain two hits with the closed one being the first.
        // 2. The ray is outside of the sphere and hits the top of the sphere.
        //    In this case the vector will contain one hit
        // 3. The ray is outside of the sphere and misses it completely.
        //    In this case None will be returned
        // 4. The ray is inside of the sphere and intersects only once
        //    In this case the vector will contain one hit
        let quadratic = self.get_quadratic_v2(r);
        quadratic.solutions.map(|solutions| {
            solutions
                .into_iter()
                .map(|t| {
                    let p = r.at(t);
                    let normal = (p - self.origin).normalise();
                    Hit{ t, p, normal }
                })
                .collect()
        })
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_intersect() {
        let ray = Ray::new(
            Point3{x: 0.0, y: 0.0, z: 0.0},
            Vec3{x: 1.0, y: 0.0, z: 0.0}
        );
        let sphere = Sphere{
            origin: Point3::new(5.0, 0.0, 0.0),
            radius: 1.0
        };
        let intersect = sphere.intersect(&ray);
        assert!(intersect);
    }
}

