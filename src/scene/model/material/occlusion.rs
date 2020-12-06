use super::Mapper;
use crate::utils::GltfData;
use image::GrayImage;
use std::sync::Arc;

#[derive(Clone, Debug)]
/// Defines the occlusion texture of a material.
pub struct Occlusion {
    /// The `occlusion_texture` refers to a texture that defines areas of the
    /// surface that are occluded from light, and thus rendered darker.
    pub texture: Arc<GrayImage>,

    /// The `occlusion_factor` is the occlusion strength to be applied to the
    /// texture value.
    pub factor: f32,

    /// Mapper to apply a scale and offset on textures.
    pub mapper: Mapper,
}

impl Occlusion {
    pub(crate) fn load(gltf_mat: &gltf::Material, data: &mut GltfData) -> Option<Self> {
        match gltf_mat.occlusion_texture() {
            Some(texture) => Some(Self {
                texture: data.load_gray_image(&texture.texture(), 0),
                factor: texture.strength(),
                mapper: Default::default(), // TODO Implem it
            }),
            None => None,
        }
    }
}
