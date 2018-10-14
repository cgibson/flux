use std::fmt;
use na::{Point3, Vector3};

use util::math::{Ray};

pub trait Shape {
    fn intersect(&self, ray: Ray) -> Option<Hit>;
}

pub struct Hit {
    pub p: Point3<f64>,
    pub t: f64,
    pub n: Vector3<f64>,
}

impl Hit {
    pub fn new(p: Point3<f64>, t: f64, n: Vector3<f64>) -> Hit {
        Hit { p: p, t: t, n: n }
    }

    pub fn zero() -> Hit {
        Hit { p: Point3::new(0.0, 0.0, 0.0),
                  t: 0.0,
                  n: Vector3::new(0.0, 0.0, 0.0) }
    }
}


impl fmt::Display for Hit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // The `f` value implements the `Write` trait, which is what the
        // write! macro is expecting. Note that this formatting ignores the
        // various flags provided to format strings.
        write!(f, "(P: {}, T: {}, N: {})", self.p, self.t, self.n)
    }
}