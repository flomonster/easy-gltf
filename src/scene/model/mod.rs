mod material;
mod vertex;

use crate::utils::*;
use cgmath::*;
use std::rc::Rc;

pub use material::Material;
pub use vertex::*;

/// Geometry to be rendered with the given material
#[derive(Clone, Debug)]
pub struct Model {
    /// List of triangles
    pub triangles: Vec<Triangle>,
    /// Material applied to the whole model
    pub material: Rc<Material>,
}

impl Model {

    fn apply_transform_position(pos: [f32; 3], transform: &Matrix4<f32>) -> Vector3<f32>
    {
        let pos = Vector4::new(pos[0], pos[1], pos[2], 1.);
        let res = transform * pos;
        Vector3::new(res.x / res.w, res.y / res.w, res.z / res.w)
    }

    fn apply_transform_vector(pos: [f32; 3], transform: &Matrix4<f32>) -> Vector3<f32>
    {
        let pos = Vector4::new(pos[0], pos[1], pos[2], 0.);
        (transform * pos).truncate()
    }

    pub(crate) fn load(
        primitive: gltf::Primitive,
        transform: &Matrix4<f32>,
        data: &GltfData,
        col: &mut Collection,
    ) -> Self {
        let buffers = &data.buffers;
        let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

        // Init vertices with the position
        let mut vertices: Vec<_> = reader
            .read_positions()
            .unwrap()
            .map(|pos| Vertex {
                position: Self::apply_transform_position(pos, transform),
                ..Default::default()
            })
            .collect();

        // Fill normals
        if let Some(normals) = reader.read_normals() {
            for (i, normal) in normals.enumerate() {
                vertices[i].normal = Self::apply_transform_vector(normal, transform).normalize();
            }
        }

        // Texture coordinates
        if let Some(tex_coords) = reader.read_tex_coords(0) {
            for (i, tex_coord) in tex_coords.into_f32().enumerate() {
                vertices[i].texture = Vector2::from(tex_coord);
            }
        }

        // TODO: Handle mode POINTS and LINES
        let mut triangles = vec![];
        if let Some(indices) = reader.read_indices() {
            let indices = indices.into_u32();
            // Read the indicies 3 by 3 to create the triangles
            for ti in indices.clone().step_by(3).zip(indices.clone().skip(1).step_by(3)).zip(indices.skip(2).step_by(3)) {
                triangles.push([vertices[ti.0.0 as usize].clone(), vertices[ti.0.1 as usize].clone(), vertices[ti.1 as usize].clone()]);
            }
        }

        Model {
            triangles,
            material: Material::load(primitive.material(), data, col),
        }
    }
}
