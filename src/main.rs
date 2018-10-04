extern crate nalgebra_glm as glm;

use glm::{DVec3};

use geometry::sphere::SphereShape;
use geometry::shape::Shape;
use util::math::Ray;

pub mod geometry;
pub mod util;

fn main() {

    let sphere = SphereShape::new( DVec3::new(0.0, 0.0, 0.0), 5.0);
    let ray = Ray::new( DVec3::new(-10.0, 0.0, 0.0), DVec3::new(1.0, 0.0, 0.0) );

    let hit_result = sphere.intersect(ray);

    match hit_result {
        Some(hit) => println!("Hello World! {}", hit),
        None => println!("Hello World. No hit?"),
    }
}
