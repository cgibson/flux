use na::{Point3, Vector3};

use util::math::{Ray};
use geometry::{Intersect, Hit};

#[derive(Serialize, Deserialize, Debug)]
pub struct Sphere {
    pub position: Point3<f64>,
    pub radius: f64,
}

impl Sphere {
    pub fn new(position: Point3<f64>, radius: f64) -> Sphere {
        Sphere {position: position, radius: radius}
    }
}

impl Intersect for Sphere {

    fn intersect(&self, ray: Ray) -> Option<Hit> {
        let mut hit: Hit = Hit::zero();

        let dir: Vector3<f64> = ray.direction;
        let pt: Point3<f64> = ray.origin;

        let a: f64 = dir.dot(&dir);
        let tmp: Vector3<f64> = pt - self.position;
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

    fn get_normal(&self, pt: Point3<f64>) -> Vector3<f64> {
        (pt - self.position).normalize()
    }
}