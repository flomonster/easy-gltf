use cgmath::*;

/// Represents the 3 vertices of a triangle.
pub type Triangle = [Vertex; 3];

/// Represents the 2 vertices of a line.
pub type Line = [Vertex; 2];

/// Contains a position, normal and texture coordinates vectors.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vertex {
    /// Position
    pub position: Vector3<f32>,
    /// Normalized normal
    pub normal: Vector3<f32>,
    /// Tangent normal
    pub tangent: Vector4<f32>,
    /// Texture coordinates
    pub tex_coords: Vector2<f32>,
}

impl Default for Vertex {
    fn default() -> Self {
        Vertex {
            position: Zero::zero(),
            normal: Zero::zero(),
            tangent: Zero::zero(),
            tex_coords: Zero::zero(),
        }
    }
}
