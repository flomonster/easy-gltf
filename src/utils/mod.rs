mod gltf_data;

pub(crate) use gltf_data::GltfData;

use cgmath::*;
use gltf::scene::Transform;

pub fn transform_to_matrix(transform: Transform) -> Matrix4<f32> {
  let tr = transform.matrix();
  Matrix4::new(
    tr[0][0], tr[0][1], tr[0][2], tr[0][3], tr[1][0], tr[1][1], tr[1][2], tr[1][3], tr[2][0],
    tr[2][1], tr[2][2], tr[2][3], tr[3][0], tr[3][1], tr[3][2], tr[3][3],
  )
}
