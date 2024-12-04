mod material;
mod mode;
mod vertex;

use crate::utils::*;
use cgmath::*;
use std::sync::Arc;

pub use material::*;
pub use mode::*;
pub use vertex::*;

/// Geometry to be rendered with the given material.
///
/// # Examples
///
/// ### Classic rendering
///
/// In most cases you want to use `triangles()`, `lines()` and `points()`
/// to get the geometry of the model.
///
/// ```
/// # use easy_gltf::*;
/// # use easy_gltf::model::Mode;
/// # let model = Model::default();
/// match model.mode() {
///   Mode::Triangles | Mode::TriangleFan | Mode::TriangleStrip => {
///     let triangles = model.triangles().unwrap();
///     // Render triangles...
///   },
///   Mode::Lines | Mode::LineLoop | Mode::LineStrip => {
///     let lines = model.lines().unwrap();
///     // Render lines...
///   }
///   Mode::Points => {
///     let points = model.points().unwrap();
///     // Render points...
///   }
/// }
/// ```
///
/// ### OpenGL style rendering
///
/// You will need the vertices and the indices if existing.
///
/// ```
/// # use easy_gltf::*;
/// # use easy_gltf::model::Mode;
/// # let model = Model::default();
/// let vertices = model. vertices();
/// let indices = model.indices();
/// match model.mode() {
///   Mode::Triangles => {
///     if let Some(indices) = indices.as_ref() {
///       // glDrawElements(GL_TRIANGLES, indices.len(), GL_UNSIGNED_INT, 0);
///     } else {
///       // glDrawArrays(GL_TRIANGLES, 0, vertices.len());
///     }
///   },
///   // ...
/// # _ => unimplemented!(),
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub struct Model {
    #[cfg(feature = "names")]
    pub(crate) mesh_name: Option<String>,
    #[cfg(feature = "extras")]
    pub(crate) mesh_extras: gltf::json::extras::Extras,
    #[cfg(feature = "extras")]
    pub(crate) primitive_extras: gltf::json::extras::Extras,

    pub(crate) primitive_index: usize,
    pub(crate) vertices: Vec<Vertex>,
    pub(crate) indices: Option<Vec<u32>>,
    pub(crate) mode: Mode,
    pub(crate) material: Arc<Material>,
    pub(crate) has_normals: bool,
    pub(crate) has_tangents: bool,
    pub(crate) has_tex_coords: bool,
    #[cfg(feature = "vertex-color")]
    pub(crate) has_colors: bool,
}

impl Model {
    #[cfg(feature = "names")]
    /// Mesh name. Requires the `names` feature.
    ///
    /// A `Model` in easy-gltf represents a primitive in gltf, so if a mesh has multiple primitives, you will
    /// get multiple `Model`s with the same name. You can use `primitive_index` to get which primitive the `Model`
    /// corresponds to.
    pub fn mesh_name(&self) -> Option<&str> {
        self.mesh_name.as_deref()
    }

    /// Index of the Primitive of the Mesh that this `Model` corresponds to.
    pub fn primitive_index(&self) -> usize {
        self.primitive_index
    }

    #[cfg(feature = "extras")]
    /// Mesh extra data. Requires the `extras` feature.
    pub fn mesh_extras(&self) -> &gltf::json::extras::Extras {
        &self.mesh_extras
    }

    #[cfg(feature = "extras")]
    /// Primitive extra data. Requires the `extras` feature.
    pub fn primitive_extras(&self) -> &gltf::json::extras::Extras {
        &self.primitive_extras
    }

    /// Material to apply to the whole model.
    pub fn material(&self) -> Arc<Material> {
        self.material.clone()
    }

    /// List of raw `vertices` of the model. You might have to use the `indices`
    /// to render the model.
    ///
    /// **Note**: If you're not rendering with **OpenGL** you probably want to use
    /// `triangles()`, `lines()` or `points()` instead.
    pub fn vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }

    /// Potential list of `indices` to render the model using raw `vertices`.
    ///
    /// **Note**: If you're **not** rendering with **OpenGL** you probably want to use
    /// `triangles()`, `lines()` or `points()` instead.
    pub fn indices(&self) -> Option<&Vec<u32>> {
        self.indices.as_ref()
    }

    /// The type of primitive to render.
    /// You have to check the `mode` to render the model correctly.
    ///
    /// Then you can either use:
    /// * `vertices()` and `indices()` to arrange the data yourself (useful for **OpenGL**).
    /// * `triangles()` or `lines()` or `points()` according to the returned mode.
    pub fn mode(&self) -> Mode {
        self.mode.clone()
    }

    /// List of triangles ready to be rendered.
    ///
    /// **Note**: This function will return an error if the mode isn't `Triangles`, `TriangleFan`
    /// or `TriangleStrip`.
    pub fn triangles(&self) -> Result<Vec<Triangle>, BadMode> {
        let mut triangles = vec![];
        let indices = (0..self.vertices.len() as u32).collect();
        let indices = self.indices().unwrap_or(&indices);

        match self.mode {
            Mode::Triangles => {
                for i in (0..indices.len()).step_by(3) {
                    triangles.push([
                        self.vertices[indices[i] as usize],
                        self.vertices[indices[i + 1] as usize],
                        self.vertices[indices[i + 2] as usize],
                    ]);
                }
            }
            Mode::TriangleStrip => {
                for i in 0..(indices.len() - 2) {
                    triangles.push([
                        self.vertices[indices[i] as usize + i % 2],
                        self.vertices[indices[i + 1 - i % 2] as usize],
                        self.vertices[indices[i + 2] as usize],
                    ]);
                }
            }
            Mode::TriangleFan => {
                for i in 1..(indices.len() - 1) {
                    triangles.push([
                        self.vertices[indices[0] as usize],
                        self.vertices[indices[i] as usize],
                        self.vertices[indices[i + 1] as usize],
                    ]);
                }
            }
            _ => return Err(BadMode { mode: self.mode() }),
        }
        Ok(triangles)
    }

    /// List of lines ready to be rendered.
    ///
    /// **Note**: This function will return an error if the mode isn't `Lines`, `LineLoop`
    /// or `LineStrip`.
    pub fn lines(&self) -> Result<Vec<Line>, BadMode> {
        let mut lines = vec![];
        let indices = (0..self.vertices.len() as u32).collect();
        let indices = self.indices().unwrap_or(&indices);
        match self.mode {
            Mode::Lines => {
                for i in (0..indices.len()).step_by(2) {
                    lines.push([
                        self.vertices[indices[i] as usize],
                        self.vertices[indices[i + 1] as usize],
                    ]);
                }
            }
            Mode::LineStrip | Mode::LineLoop => {
                for i in 0..(indices.len() - 1) {
                    lines.push([
                        self.vertices[indices[i] as usize],
                        self.vertices[indices[i + 1] as usize],
                    ]);
                }
            }
            _ => return Err(BadMode { mode: self.mode() }),
        }
        if self.mode == Mode::LineLoop {
            lines.push([
                self.vertices[indices[0] as usize],
                self.vertices[indices[indices.len() - 1] as usize],
            ]);
        }

        Ok(lines)
    }

    /// List of points ready to be renderer.
    ///
    /// **Note**: This function will return an error if the mode isn't `Points`.
    pub fn points(&self) -> Result<&Vec<Vertex>, BadMode> {
        match self.mode {
            Mode::Points => Ok(&self.vertices),
            _ => Err(BadMode { mode: self.mode() }),
        }
    }

    /// Indicate if the vertices contains normal information.
    ///
    /// **Note**: If this function return `false` all vertices has a normal field
    /// initialized to `zero`.
    pub fn has_normals(&self) -> bool {
        self.has_normals
    }

    /// Indicate if the vertices contains tangents information.
    ///
    /// **Note**: If this function return `false` all vertices has a tangent field
    /// initialized to `zero`.
    pub fn has_tangents(&self) -> bool {
        self.has_tangents
    }

    /// Indicate if the vertices contains texture coordinates information.
    ///
    /// **Note**: If this function return `false` all vertices has a tex_coord field
    /// initialized to `zero`.
    pub fn has_tex_coords(&self) -> bool {
        self.has_tex_coords
    }

    /// Indicate if the vertices contains color information.
    /// Requires the `vertex-color` feature.
    ///
    /// **Note**: If this function return `false` all vertices has a color field
    /// initialized to `1`.
    #[cfg(feature = "vertex-color")]
    pub fn has_colors(&self) -> bool {
        self.has_colors
    }

    fn apply_transform_position(pos: [f32; 3], transform: &Matrix4<f32>) -> Vector3<f32> {
        let pos = Vector4::new(pos[0], pos[1], pos[2], 1.);
        let res = transform * pos;
        Vector3::new(res.x / res.w, res.y / res.w, res.z / res.w)
    }

    fn apply_transform_vector(vec: [f32; 3], transform: &Matrix4<f32>) -> Vector3<f32> {
        let vec = Vector4::new(vec[0], vec[1], vec[2], 0.);
        (transform * vec).truncate()
    }

    fn apply_transform_tangent(tangent: [f32; 4], transform: &Matrix4<f32>) -> Vector4<f32> {
        let tang = Vector4::new(tangent[0], tangent[1], tangent[2], 0.);
        let mut tang = transform * tang;
        tang[3] = tangent[3];
        tang
    }

    pub(crate) fn load(
        mesh: &gltf::Mesh,
        primitive_index: usize,
        primitive: gltf::Primitive,
        transform: &Matrix4<f32>,
        data: &mut GltfData,
    ) -> Self {
        #[cfg(not(feature = "names"))]
        {
            let _ = mesh;
        }

        let buffers = &data.buffers;
        let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
        let indices = reader
            .read_indices()
            .map(|indices| indices.into_u32().collect());

        // Init vertices with the position
        let mut vertices: Vec<_> = reader
            .read_positions()
            .unwrap_or_else(|| panic!("The model primitive doesn't contain positions"))
            .map(|pos| Vertex {
                position: Self::apply_transform_position(pos, transform),
                ..Default::default()
            })
            .collect();

        // Fill normals
        let has_normals = if let Some(normals) = reader.read_normals() {
            for (i, normal) in normals.enumerate() {
                vertices[i].normal = Self::apply_transform_vector(normal, transform).normalize();
            }
            true
        } else {
            false
        };

        // Fill tangents
        let has_tangents = if let Some(tangents) = reader.read_tangents() {
            for (i, tangent) in tangents.enumerate() {
                let tangent = Self::apply_transform_tangent(tangent, transform);
                vertices[i].tangent = tangent.truncate().normalize().extend(tangent.w);
            }
            true
        } else {
            false
        };

        // Texture coordinates
        let has_tex_coords = if let Some(tex_coords) = reader.read_tex_coords(0) {
            for (i, tex_coords) in tex_coords.into_f32().enumerate() {
                vertices[i].tex_coords = Vector2::from(tex_coords);
            }
            true
        } else {
            false
        };

        // Colors
        #[cfg(feature = "vertex-color")]
        let has_colors = if let Some(colors) = reader.read_colors(0) {
            for (i, color) in colors.into_rgba_u16().enumerate() {
                vertices[i].color = Vector4::from(color);
            }
            true
        } else {
            false
        };

        Model {
            #[cfg(feature = "names")]
            mesh_name: mesh.name().map(String::from),
            #[cfg(feature = "extras")]
            mesh_extras: mesh.extras().clone(),
            #[cfg(feature = "extras")]
            primitive_extras: primitive.extras().clone(),
            primitive_index,
            vertices,
            indices,
            material: Material::load(primitive.material(), data),
            mode: primitive.mode().into(),
            has_normals,
            has_tangents,
            has_tex_coords,
            #[cfg(feature = "vertex-color")]
            has_colors,
        }
    }
}
