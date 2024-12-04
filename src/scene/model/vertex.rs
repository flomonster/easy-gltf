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
    /// The w component is the handedness of the tangent basis (can be -1 or 1)
    pub tangent: Vector4<f32>,
    /// Texture coordinates
    pub tex_coords: Vector2<f32>,
    /// Vertex color, known to be compatible with Blender 4 exported models
    #[cfg(feature = "vertex-color")]
    pub color: Vector4<u16>, // Blender exported glTF uses componentType 5123 (UNSIGNED_SHORT)
}

impl Default for Vertex {
    fn default() -> Self {
        Vertex {
            position: Zero::zero(),
            normal: Zero::zero(),
            tangent: Zero::zero(),
            tex_coords: Zero::zero(),
            #[cfg(feature = "vertex-color")]
            color: Vector4::new(0, 0, 0, 0),
        }
    }
}
