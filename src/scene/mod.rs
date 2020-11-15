mod camera;
mod light;
mod model;

use crate::utils::*;
use crate::GltfData;
pub use camera::Camera;
pub use light::Light;
pub use model::*;

use cgmath::*;
use gltf::scene::Node;
use gltf::Result;

#[derive(Clone, Debug)]
pub struct Scene {
    pub models: Vec<Model>,
    pub cameras: Vec<Camera>,
    pub lights: Vec<Light>,
}

impl Scene {
    pub(crate) fn load(
        gltf_scene: gltf::Scene,
        data: &GltfData,
        col: &mut Collection,
    ) -> Result<Self> {
        let mut scene = Self::default();
        for node in gltf_scene.nodes() {
            scene.read_node(&node, &One::one(), data, col)?;
        }
        Ok(scene)
    }

    fn read_node(
        &mut self,
        node: &Node,
        parent_transform: &Matrix4<f32>,
        data: &GltfData,
        col: &mut Collection,
    ) -> Result<()> {
        // Compute transform of the current node
        let transform = parent_transform * transform_to_matrix(node.transform());

        // Recurse on children
        for child in node.children() {
            self.read_node(&child, &transform, data, col)?;
        }

        // Load camera
        if let Some(camera) = node.camera() {
            self.cameras.push(Camera::load(camera, &transform));
        }

        // Load light
        if let Some(light) = node.light() {
            self.lights.push(Light::load(light, &transform));
        }

        // Load model
        if let Some(mesh) = node.mesh() {
            for primitive in mesh.primitives() {
                self.models
                    .push(Model::load(primitive, &transform, data, col));
            }
        }
        Ok(())
    }
}

impl Default for Scene {
    fn default() -> Self {
        Scene {
            models: vec![],
            cameras: vec![],
            lights: vec![],
        }
    }
}
