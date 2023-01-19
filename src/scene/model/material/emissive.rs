use crate::utils::GltfData;
use cgmath::*;
use image::RgbImage;
use std::sync::Arc;

#[derive(Clone, Debug)]
/// The emissive color of the material.
pub struct Emissive {
    /// The `emissive_texture` refers to a texture that may be used to illuminate parts of the
    /// model surface: It defines the color of the light that is emitted from the surface
    pub texture: Option<Arc<RgbImage>>,

    /// The `emissive_factor` contains scaling factors for the red, green and
    /// blue components of this texture.
    pub factor: Vector3<f32>,
}

impl Emissive {
    pub(crate) fn load(gltf_mat: &gltf::Material, data: &mut GltfData) -> Self {
        Self {
            texture: gltf_mat
                .emissive_texture()
                .map(|texture| data.load_rgb_image(&texture.texture())),
            factor: gltf_mat.emissive_factor().into(),
        }
    }
}

impl Default for Emissive {
    fn default() -> Self {
        Self {
            texture: None,
            factor: Vector3::zero(),
        }
    }
}
