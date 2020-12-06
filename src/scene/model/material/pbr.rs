use super::Mapper;
use crate::utils::GltfData;
use cgmath::*;
use image::{GrayImage, RgbaImage};
use std::sync::Arc;

#[derive(Clone, Debug)]
/// A set of parameter values that are used to define the metallic-roughness
/// material model from Physically-Based Rendering (PBR) methodology.
pub struct PbrMaterial {
    /// The `base_color_factor` contains scaling factors for the red, green,
    /// blue and alpha component of the color. If no texture is used, these
    /// values will define the color of the whole object in **RGB** color space.
    pub base_color_factor: Vector4<f32>,

    /// The `base_color_texture` is the main texture that will be applied to the
    /// object.
    ///
    /// The texture contains RGB(A) components in **sRGB** color space.
    pub base_color_texture: Option<Arc<RgbaImage>>,

    /// Contains the metalness value
    pub metallic_texture: Option<Arc<GrayImage>>,

    /// `metallic_factor` is multiply to the `metallic_texture` value. If no
    /// texture is given, then the factor define the metalness for the whole
    /// object.
    pub metallic_factor: f32,

    /// Contains the roughness value
    pub roughness_texture: Option<Arc<GrayImage>>,

    /// `roughness_factor` is multiply to the `roughness_texture` value. If no
    /// texture is given, then the factor define the roughness for the whole
    /// object.
    pub roughness_factor: f32,

    /// Mapper to apply a scale and offset on textures.
    pub mapper: Mapper,
}

impl PbrMaterial {
    pub(crate) fn load(pbr: gltf::material::PbrMetallicRoughness, data: &mut GltfData) -> Self {
        let mut material = Self::default();
        material.base_color_factor = pbr.base_color_factor().into();
        if let Some(texture) = pbr.base_color_texture() {
            material.base_color_texture = Some(data.load_base_color_image(&texture.texture()));
        }

        material.roughness_factor = pbr.roughness_factor();
        material.metallic_factor = pbr.metallic_factor();

        if let Some(texture) = pbr.metallic_roughness_texture() {
            material.metallic_texture = Some(data.load_gray_image(&texture.texture(), 2));
            material.roughness_texture = Some(data.load_gray_image(&texture.texture(), 1));
        }

        material
    }
}

impl Default for PbrMaterial {
    fn default() -> Self {
        PbrMaterial {
            base_color_factor: Vector4::new(1., 1., 1., 1.),
            base_color_texture: None,
            metallic_factor: 0.,
            metallic_texture: None,
            roughness_factor: 0.,
            roughness_texture: None,
            mapper: Default::default(),
        }
    }
}
