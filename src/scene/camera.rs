
use na::{Point3, Vector3};
use util::math::Ray;

pub struct Camera {
    pub position: Point3<f64>,
}

impl Camera {
    pub fn new(position: Point3<f64>) -> Camera {
        Camera {position: position}
    }
}

