use cgmath::*;

#[derive(Clone, Debug)]
/// Define an offset and a scale to apply to texture coordinates.
pub struct Mapper {
    /// Offset of texture coordinates.
    pub offset: Vector2<f32>,
    /// Scale of texture coordinates.
    pub scale: Vector2<f32>,
}

impl Default for Mapper {
    fn default() -> Self {
        Mapper {
            offset: Vector2::zero(),
            scale: Vector2::new(1., 1.),
        }
    }
}
