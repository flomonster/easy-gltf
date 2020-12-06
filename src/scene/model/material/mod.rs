mod emissive;
mod mapper;
mod normal;
mod occlusion;
mod pbr;

use crate::utils::*;
use cgmath::*;
use std::sync::Arc;

pub use emissive::Emissive;
pub use mapper::Mapper;
pub use normal::NormalMap;
pub use occlusion::Occlusion;
pub use pbr::PbrMaterial;

/// Contains material properties of models.
#[derive(Clone, Debug)]
pub struct Material {
    /// Parameter values that define the metallic-roughness material model from
    /// Physically-Based Rendering (PBR) methodology.
    pub pbr: PbrMaterial,

    /// Defines the normal texture of a material.
    pub normal: Option<NormalMap>,

    /// Defines the occlusion texture of a material.
    pub occlusion: Option<Occlusion>,

    /// The emissive color of the material.
    pub emissive: Emissive,
}

impl Material {
    /// Get the color base Rgb(A) (in RGB-color space) of the material given a
    /// texture coordinate. If no `base_color_texture` is available then the
    /// `base_color_factor` is returned.
    ///
    /// **Important**: `tex_coords` must contain values between `[0., 1.]`
    /// otherwise the function will fail.
    pub fn get_base_color_alpha(&self, tex_coords: Vector2<f32>) -> Vector4<f32> {
        let mut res = self.pbr.base_color_factor;
        if let Some(texture) = &self.pbr.base_color_texture {
            let coords = tex_coords.mul_element_wise(Vector2::new(
                texture.width() as f32,
                texture.height() as f32,
            ));
            let px_u = texture[(coords.x as u32, coords.y as u32)];
            // Transform to float
            let mut px_f = Vector4::new(0., 0., 0., 0.);
            for i in 0..4 {
                px_f[i] = (px_u[i] as f32) / 255.;
            }
            // Convert sRGB to RGB
            let pixel = Vector4::new(px_f.x.powf(2.2), px_f.y.powf(2.2), px_f.z.powf(2.2), px_f.w);
            // Multiply to the scale factor
            for i in 0..4 {
                res[i] *= pixel[i];
            }
        }
        res
    }

    /// Get the color base Rgb (in RGB-color space) of the material given a
    /// texture coordinate. If no `base_color_texture` is available then the
    /// `base_color_factor` is returned.
    ///
    /// **Important**: `tex_coords` must contain values between `[0., 1.]`
    /// otherwise the function will fail.
    pub fn get_base_color(&self, tex_coords: Vector2<f32>) -> Vector3<f32> {
        self.get_base_color_alpha(tex_coords).truncate()
    }

    /// Get the metallic value of the material given a texture coordinate. If no
    /// `metallic_texture` is available then the `metallic_factor` is returned.
    ///
    /// **Important**: `tex_coords` must contain values between `[0., 1.]`
    /// otherwise the function will fail.
    pub fn get_metallic(&self, tex_coords: Vector2<f32>) -> f32 {
        self.pbr.metallic_factor
            * if let Some(texture) = &self.pbr.metallic_texture {
                let coords = tex_coords.mul_element_wise(Vector2::new(
                    texture.width() as f32,
                    texture.height() as f32,
                ));
                (texture[(coords.x as u32, coords.y as u32)][0] as f32) / 255.
            } else {
                1.
            }
    }

    /// Get the roughness value of the material given a texture coordinate. If no
    /// `roughness_texture` is available then the `roughness_factor` is returned.
    ///
    /// **Important**: `tex_coords` must contain values between `[0., 1.]`
    /// otherwise the function will fail.
    pub fn get_roughness(&self, tex_coords: Vector2<f32>) -> f32 {
        self.pbr.roughness_factor
            * if let Some(texture) = &self.pbr.roughness_texture {
                let coords = tex_coords.mul_element_wise(Vector2::new(
                    texture.width() as f32,
                    texture.height() as f32,
                ));
                (texture[(coords.x as u32, coords.y as u32)][0] as f32) / 255.
            } else {
                1.
            }
    }

    /// Get the normal vector of the material given a texture coordinate. If no
    /// `normal_texture` is available then `None` is returned.
    ///
    /// **Important**: `tex_coords` must contain values between `[0., 1.]`
    /// otherwise the function will fail.
    pub fn get_normal(&self, tex_coords: Vector2<f32>) -> Option<Vector3<f32>> {
        let normal = self.normal.as_ref()?;
        let coords = tex_coords.mul_element_wise(Vector2::new(
            normal.texture.width() as f32,
            normal.texture.height() as f32,
        ));
        let pixel = normal.texture[(coords.x as u32, coords.y as u32)];
        Some(
            normal.factor
                * Vector3::new(
                    (pixel[0] as f32) / 127.5 - 1.,
                    (pixel[1] as f32) / 127.5 - 1.,
                    (pixel[2] as f32) / 127.5 - 1.,
                ),
        )
    }

    /// Get the occlusion value of the material given a texture coordinate. If no
    /// `occlusion_texture` is available then `None` is returned.
    ///
    /// **Important**: `tex_coords` must contain values between `[0., 1.]`
    /// otherwise the function will fail.
    pub fn get_occlusion(&self, tex_coords: Vector2<f32>) -> Option<f32> {
        let occlusion = self.occlusion.as_ref()?;
        let coords = tex_coords.mul_element_wise(Vector2::new(
            occlusion.texture.width() as f32,
            occlusion.texture.height() as f32,
        ));
        Some(
            occlusion.factor * (occlusion.texture[(coords.x as u32, coords.y as u32)][0] as f32)
                / 255.,
        )
    }

    /// Get the emissive color Rgb of the material given a texture coordinate.
    /// If no `emissive_texture` is available then the `emissive_factor` is
    /// returned.
    ///
    /// **Important**: `tex_coords` must contain values between `[0., 1.]`
    /// otherwise the function will fail.
    pub fn get_emissive(&self, tex_coords: Vector2<f32>) -> Vector3<f32> {
        let mut res = self.emissive.factor;
        if let Some(texture) = &self.emissive.texture {
            let coords = tex_coords.mul_element_wise(Vector2::new(
                texture.width() as f32,
                texture.height() as f32,
            ));
            let pixel = texture[(coords.x as u32, coords.y as u32)];
            for i in 0..3 {
                res[i] *= (pixel[i] as f32) / 255.;
            }
        }
        res
    }

    pub(crate) fn load(gltf_mat: gltf::Material, data: &mut GltfData) -> Arc<Self> {
        if let Some(material) = data.materials.get(&gltf_mat.index()) {
            return material.clone();
        }

        let material = Arc::new(Material {
            pbr: PbrMaterial::load(gltf_mat.pbr_metallic_roughness(), data),
            normal: NormalMap::load(&gltf_mat, data),
            occlusion: Occlusion::load(&gltf_mat, data),
            emissive: Emissive::load(&gltf_mat, data),
        });

        // Add to the collection
        data.materials.insert(gltf_mat.index(), material.clone());
        material
    }
}

impl Default for Material {
    fn default() -> Self {
        Material {
            pbr: Default::default(),
            normal: None,
            occlusion: None,
            emissive: Default::default(),
        }
    }
}
