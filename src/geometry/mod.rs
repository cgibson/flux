use util::math::Ray;
use na::{Point3, Vector3};

mod sphere;
mod object;
mod intersect;


pub use self::sphere::Sphere;
pub use self::object::SceneObject;
pub use self::intersect::{Intersect, Hit};


#[derive(Serialize, Deserialize, Debug)]
pub struct Pose {
    pub position: Point3<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Shape {
    Sphere(Sphere),
}

impl Shape {
	pub fn new_sphere(position: Point3<f64>, radius: f64) -> Shape {
		Shape::Sphere( Sphere{position: position, radius: radius} )
	}
}

impl Intersect for Shape {
	fn intersect(&self, ray: Ray) -> Option<Hit>{
	    match self {
	    	Shape::Sphere(inner) => inner.intersect(ray)
	    }
	}

	fn get_normal(&self, pt: Point3<f64>) -> Vector3<f64> {
		match self {
			Shape::Sphere(inner) => inner.get_normal(pt)
		}
	}
}