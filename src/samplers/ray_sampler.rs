use na::{Vector3, Point3};

use util::math::Ray;
use util::color::{Spectrum,Pixel};
use scene::camera::Camera;

pub struct SamplePartial {
    pub depth: i32,
    pub max_depth: i32,
    pub color: Spectrum,
    pub pixel: Pixel,
}

pub trait RaySampler {
    fn generate_rays(&self, camera: Camera) -> Vec<Ray>;
    fn foreach_rays<F>(&self, camera: Camera, closure: F) -> Vec<SamplePartial>
        where F : Fn(Ray) -> SamplePartial {

        self.generate_rays(camera).into_iter().map(closure).collect()

    }
}

struct SimpleSampler {

}

impl RaySampler for SimpleSampler {
    fn generate_rays(&self, camera: Camera) -> Vec<Ray> {
        let center: Point3<f64> = camera.position;
        let mut rays = Vec::new();
        for x in -1i32..1i32 {
            for y in -1i32..1i32 {
                let offset = Vector3::new(x as f64, y as f64, 0.0);
                rays.push(Ray::new(center + offset,
                        Vector3::new(1.0, 0.0, 0.0)));
            }
        }
        return rays;
    }
}