use crate::utils::*;
use base64;
use cgmath::*;
use gltf::image::Source;
use image::*;
use image::{DynamicImage, GrayImage, RgbImage, RgbaImage};
use std::rc::Rc;

/// Contains material properties of models.
#[derive(Clone, Debug)]
pub struct Material {
    /// The `base_color_factor` contains scaling factors for the red, green,
    /// blue and alpha component of the color. If no texture is used, these
    /// values will define the color of the whole object.
    pub base_color_factor: Vector4<f32>,

    /// The `base_color_texture` is the main texture that will be applied to the
    /// object.
    pub base_color_texture: Option<Rc<RgbaImage>>,

    /// The alpha cutoff value of the material.
    pub alpha_cutoff: f32,

    /// Contains the metalness value
    pub metallic_texture: Option<Rc<GrayImage>>,

    /// `metallic_factor` is multiply to the `metallic_texture` value. If no
    /// texture is given, then the factor define the metalness for the whole
    /// object.
    pub metallic_factor: f32,

    /// Contains the roughness value
    pub roughness_texture: Option<Rc<GrayImage>>,

    /// `roughness_factor` is multiply to the `roughness_texture` value. If no
    /// texture is given, then the factor define the roughness for the whole
    /// object.
    pub roughness_factor: f32,

    /// A tangent space normal map.
    /// The texture contains RGB components in linear space. Each texel
    /// represents the XYZ components of a normal vector in tangent space.
    ///
    /// * Red [0 to 255] maps to X [-1 to 1].
    /// * Green [0 to 255] maps to Y [-1 to 1].
    /// * Blue [128 to 255] maps to Z [1/255 to 1].
    ///
    /// The normal vectors use OpenGL conventions where +X is right, +Y is up,
    /// and +Z points toward the viewer.
    pub normal_texture: Option<Rc<RgbImage>>,

    /// The `normal_factor` is the normal strength to be applied to the
    /// texture value.
    pub normal_factor: f32,

    /// The `occlusion_texture` refers to a texture that defines areas of the
    /// surface that are occluded from light, and thus rendered darker.
    pub occlusion_texture: Option<Rc<GrayImage>>,

    /// The `occlusion_factor` is the occlusion strength to be applied to the
    /// texture value.
    pub occlusion_factor: f32,

    /// The `emissive_factor` contains scaling factors for the red, green and
    /// blue components of this texture.
    pub emissive_factor: Vector3<f32>,

    /// The `emissive_texture` refers to a texture that may be used to illuminate parts of the
    /// model surface: It defines the color of the light that is emitted from the surface
    pub emissive_texture: Option<Rc<RgbImage>>,
}

impl Material {
    pub(crate) fn load(
        gltf_mat: gltf::Material,
        data: &GltfData,
        col: &mut Collection,
    ) -> Rc<Self> {
        if let Some(material) = col.materials.get(&gltf_mat.index()) {
            return material.clone();
        }
        let mut material = Self::default();

        let pbr = gltf_mat.pbr_metallic_roughness();
        material.base_color_factor = pbr.base_color_factor().into();
        if let Some(texture) = pbr.base_color_texture() {
            material.base_color_texture =
                Some(Self::load_base_color_image(&texture.texture(), data, col));
        }

        material.roughness_factor = pbr.roughness_factor();
        material.metallic_factor = pbr.metallic_factor();

        if let Some(texture) = pbr.metallic_roughness_texture() {
            material.metallic_texture =
                Some(Self::load_gray_image(&texture.texture(), data, col, 2));
            material.roughness_texture =
                Some(Self::load_gray_image(&texture.texture(), data, col, 1));
        }

        material.alpha_cutoff = gltf_mat.alpha_cutoff();

        if let Some(texture) = gltf_mat.normal_texture() {
            material.normal_factor = texture.scale();
            material.normal_texture = Some(Self::load_rgb_image(&texture.texture(), data, col));
        }

        if let Some(texture) = gltf_mat.occlusion_texture() {
            material.normal_factor = texture.strength();
            material.occlusion_texture =
                Some(Self::load_gray_image(&texture.texture(), data, col, 0));
        }

        material.emissive_factor = gltf_mat.emissive_factor().into();
        if let Some(texture) = gltf_mat.emissive_texture() {
            material.emissive_texture = Some(Self::load_rgb_image(&texture.texture(), data, col));
        }

        // Add to the collection
        let material = Rc::new(material);
        col.materials.insert(gltf_mat.index(), material.clone());
        material
    }

    fn load_rgb_image(
        texture: &gltf::Texture<'_>,
        data: &GltfData,
        col: &mut Collection,
    ) -> Rc<RgbImage> {
        if let Some(image) = col.rgb_images.get(&texture.index()) {
            return image.clone();
        }
        let img = Rc::new(Self::load_texture(&texture, data).to_rgb());
        col.rgb_images.insert(texture.index(), img.clone());
        img
    }

    fn load_base_color_image(
        texture: &gltf::Texture<'_>,
        data: &GltfData,
        col: &mut Collection,
    ) -> Rc<RgbaImage> {
        if let Some(image) = col.rgba_images.get(&texture.index()) {
            return image.clone();
        }
        let img = Rc::new(Self::load_texture(&texture, data).to_rgba());
        col.rgba_images.insert(texture.index(), img.clone());
        img
    }

    fn load_gray_image(
        texture: &gltf::Texture<'_>,
        data: &GltfData,
        col: &mut Collection,
        channel: usize,
    ) -> Rc<GrayImage> {
        if let Some(image) = col.gray_images.get(&texture.index()) {
            return image.clone();
        }
        let img = Self::load_texture(&texture, data).to_rgba();
        let mut extract_img = GrayImage::new(img.width(), img.height());
        for (x, y, px) in img.enumerate_pixels() {
            extract_img[(x, y)][0] = px[channel];
        }
        let img = Rc::new(extract_img);
        col.gray_images.insert(texture.index(), img.clone());
        img
    }

    fn load_texture(texture: &gltf::Texture<'_>, data: &GltfData) -> DynamicImage {
        let g_img = texture.source();
        let buffers = &data.buffers;
        match g_img.source() {
            Source::View { view, mime_type } => {
                let parent_buffer_data = &buffers[view.buffer().index()].0;
                let data = &parent_buffer_data[view.offset()..view.offset() + view.length()];
                let mime_type = mime_type.replace("/", ".");
                image::load_from_memory_with_format(
                    data,
                    ImageFormat::from_path(mime_type).unwrap(),
                )
                .unwrap()
            }
            Source::Uri { uri, mime_type } => {
                if uri.starts_with("data:") {
                    let encoded = uri.split(',').nth(1).unwrap();
                    let data = base64::decode(&encoded).unwrap();
                    let mime_type = if let Some(ty) = mime_type {
                        ty
                    } else {
                        uri.split(',')
                            .nth(0)
                            .unwrap()
                            .split(':')
                            .nth(1)
                            .unwrap()
                            .split(';')
                            .nth(0)
                            .unwrap()
                    };
                    let mime_type = mime_type.replace("/", ".");
                    image::load_from_memory_with_format(
                        &data,
                        ImageFormat::from_path(mime_type).unwrap(),
                    )
                    .unwrap()
                } else {
                    let path = data.base_dir.join(uri);
                    open(path).unwrap()
                }
            }
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Material {
            base_color_factor: Vector4::new(1., 1., 1., 1.),
            base_color_texture: None,
            alpha_cutoff: 1.,
            metallic_factor: 0.,
            metallic_texture: None,
            roughness_factor: 0.,
            roughness_texture: None,
            normal_factor: 0.,
            normal_texture: None,
            occlusion_factor: 0.,
            occlusion_texture: None,
            emissive_factor: Vector3::zero(),
            emissive_texture: None,
        }
    }
}
