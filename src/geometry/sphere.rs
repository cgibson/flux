use glm::{DVec3};

use util::math::{Ray};
use geometry::shape::{Shape, Hit};


pub struct SphereShape {
    pub position: DVec3,
    pub radius: f64,
}

impl SphereShape {
    pub fn new(position: DVec3, radius: f64) -> SphereShape {
        SphereShape {position: position, radius: radius}
    }

    pub fn get_normal(&self, pt: DVec3) -> DVec3 {
        (pt - self.position).normalize()
    }
}

impl Shape for SphereShape {

    fn intersect(&self, ray: Ray) -> Option<Hit> {
        let mut hit: Hit = Hit::zero();

        let dir: DVec3 = ray.direction;
        let pt: DVec3 = ray.origin;

        let a: f64 = dir.dot(&dir);
        let tmp: DVec3 = pt - self.position;
        let b: f64 = 2.0 * dir.dot(&tmp);
        let c: f64 = tmp.dot(&tmp) - self.radius.powi(2);

        let discriminent = b.powi(2) - (4.0 * a * c);
        println!("discriminent: {}", discriminent);
        if discriminent > 0.0 {
            let t1: f64 = (-b - discriminent.sqrt()) / (2.0 * a);
            let t2: f64 = (-b + discriminent.sqrt()) / (2.0 * a);
     
            println!("t1: {}, t2: {}", t1, t2);       
            hit.t = if t1 > 1e-5 {t1} else {t2}
        } else if discriminent == 0.0 {
            hit.t = -b / (2.0 * a);
        } else {
            return None;
        }

        hit.p = pt + dir.normalize() * hit.t;
        hit.n = self.get_normal(hit.p);

        Some(hit)
    }
}