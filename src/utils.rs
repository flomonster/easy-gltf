use crate::Material;
use cgmath::*;
use gltf::scene::Transform;
use image::{GrayImage, RgbImage, RgbaImage};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Helps to simplify the signature of import related functions.
pub struct GltfData {
    pub doc: gltf::Document,
    pub buffers: Vec<gltf::buffer::Data>,
    pub images: Vec<gltf::image::Data>,
    pub base_dir: PathBuf,
}

impl GltfData {
    pub fn new<P>(
        doc: gltf::Document,
        buffers: Vec<gltf::buffer::Data>,
        images: Vec<gltf::image::Data>,
        path: P,
    ) -> Self
    where
        P: AsRef<Path>,
    {
        let mut base_dir = PathBuf::from(path.as_ref());
        base_dir.pop();
        GltfData {
            doc,
            buffers,
            images,
            base_dir,
        }
    }
}

#[derive(Debug, Default)]
pub struct Collection {
    pub materials: HashMap<Option<usize>, Arc<Material>>,
    pub rgb_images: HashMap<usize, Arc<RgbImage>>,
    pub rgba_images: HashMap<usize, Arc<RgbaImage>>,
    pub gray_images: HashMap<(usize, usize), Arc<GrayImage>>,
}

pub fn transform_to_matrix(transform: Transform) -> Matrix4<f32> {
    let tr = transform.matrix();
    Matrix4::new(
        tr[0][0], tr[0][1], tr[0][2], tr[0][3], tr[1][0], tr[1][1], tr[1][2], tr[1][3], tr[2][0],
        tr[2][1], tr[2][2], tr[2][3], tr[3][0], tr[3][1], tr[3][2], tr[3][3],
    )
}
