use cgmath::*;

/// Represents the 3 vertices of a triangle.
pub type Triangle = [Vertex; 3];

/// Represents the 2 vertices of a line.
pub type Line = [Vertex; 2];

/// Contains a position, normal and texture coordinates vectors.
#[derive(Clone, Debug, PartialEq)]
pub struct Vertex {
    /// Position
    pub position: Vector3<f32>,
    /// Normalized normal
    pub normal: Vector3<f32>,
    /// Texture coordinates
    pub texture: Vector2<f32>,
}

impl Default for Vertex {
    fn default() -> Self {
        Vertex {
            position: Zero::zero(),
            normal: Zero::zero(),
            texture: Zero::zero(),
        }
    }
}
