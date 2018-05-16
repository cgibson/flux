extern crate nalgebra as na;

use na::{Vector3, Point3};

use geometry::sphere::SphereShape;
use geometry::shape::Shape;
use util::math::Ray;

pub mod geometry;
pub mod util;

fn main() {

    let sphere = SphereShape::new( Point3::new(0.0, 0.0, 0.0), 5.0);
    let ray = Ray::new( Point3::new(-10.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0) );

    let hit_result = sphere.intersect(ray);

    match hit_result {
        Some(hit) => println!("Hello World! {}", hit),
        None => println!("Hello World. No hit?"),
    }
}
