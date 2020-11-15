use crate::Material;
use cgmath::*;
use gltf::scene::Transform;
use image::{GrayImage, RgbImage, RgbaImage};
use std::collections::HashMap;
use std::rc::Rc;

#[macro_export]
macro_rules! assert_delta {
    ($x:expr, $y:expr, $d:expr) => {
        if !($x - $y < $d || $y - $x < $d) {
            panic!();
        }
    };
}

/// Helps to simplify the signature of import related functions.
pub struct GltfData {
    pub doc: gltf::Document,
    pub buffers: Vec<gltf::buffer::Data>,
    pub images: Vec<gltf::image::Data>,
}

#[derive(Debug, Default)]
pub struct Collection {
    pub materials: HashMap<usize, Rc<Material>>,
    pub rgb_images: HashMap<usize, Rc<RgbImage>>,
    pub rgba_images: HashMap<usize, Rc<RgbaImage>>,
    pub gray_images: HashMap<usize, Rc<GrayImage>>,
}

pub fn transform_to_matrix(transform: Transform) -> Matrix4<f32> {
    let tr = transform.matrix();
    Matrix4::new(
        tr[0][0], tr[0][1], tr[0][2], tr[0][3], tr[1][0], tr[1][1], tr[1][2], tr[1][3], tr[2][0],
        tr[2][1], tr[2][2], tr[2][3], tr[3][0], tr[3][1], tr[3][2], tr[3][3],
    )
}
