pub mod camera;
pub mod scene;
// pub mod object;

use geometry::SceneObject;

#[derive(Serialize, Deserialize, Debug)]
pub struct Scene {
    pub objects: Vec<SceneObject>,
}