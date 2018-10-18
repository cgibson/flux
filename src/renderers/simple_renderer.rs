use renderers::generic_renderer::GenericRenderer;
use scene::Scene;
use scene::camera::Camera;

struct SimpleRenderer {
    scene: Scene,
    camera: Camera,
}

impl SimpleRenderer {
    fn new(scene: Scene, camera: Camera) -> SimpleRenderer {
        SimpleRenderer {scene: scene, camera: camera}
    }
}

impl GenericRenderer for SimpleRenderer {
    fn render(&self) {
        
    }
}