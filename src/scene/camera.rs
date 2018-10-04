
use glm::{DVec3};
use util::math::Ray;

pub struct Camera {
    pub position: DVec3,
}

impl Camera {
    pub fn new(position: DVec3) -> Camera {
        Camera {position: position}
    }
}

