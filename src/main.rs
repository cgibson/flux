#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate nalgebra as na;
use geometry::Pose;
use material::Material;
use geometry::SceneObject;
use na::{Vector3, Point3};

use std::fs::File;
use std::io::Read;
use std::error::Error;

use scene::Scene;
use geometry::Shape;
use geometry::Sphere;
use geometry::Intersect;
use util::math::Ray;

pub mod scene;
pub mod geometry;
pub mod util;
pub mod material;


fn main() -> Result<(), Box<Error>> {

    let sphere = Shape::new_sphere(Point3::new(0.0, 0.0, 0.0), 5.0);
    let ray = Ray::new( Point3::new(-10.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0) );

    let hit_result = sphere.intersect(ray);

    match hit_result {
        Some(hit) => println!("Hello World! {:#}", hit),
        None => println!("Hello World. No hit?"),
    }


    // let mut scene = Scene { objects:vec!() };

    // scene.objects.push(
    //     SceneObject {
    //         shape: Shape::new_sphere(Point3::new(0.0, 0.0, 0.0), 5.0),
    //         material: Material {},
    //         pose: Pose { position: Point3::new(0.0, 0.0, 0.0) },
    //     }
    // );


    let mut f = File::open("test.scene.pbtxt").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let data: Scene = serde_json::from_str(&contents)?;
    println!("{:#?}", data);


    // let point = Point { x: 1, y: 2, val: Value::Something(55.0), p: Point3::new(1.0,2.0,3.0) };

    // // Convert the Point to a JSON string.
    // let serialized = serde_json::to_string(&scene).unwrap();

    // // Prints serialized = {"x":1,"y":2}
    // println!("serialized = {}", serde_json::to_string_pretty(&scene).unwrap());

    // // Convert the JSON string back to a Point.
    // let deserialized: Point = serde_json::from_str(&serialized).unwrap();

    // // Prints deserialized = Point { x: 1, y: 2 }
    // println!("deserialized = {:?}", deserialized);

    Ok(())
}
