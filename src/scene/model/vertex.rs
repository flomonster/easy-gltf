use cgmath::*;

pub type Triangle = [Vertex; 3];

#[derive(Clone, Debug, PartialEq)]
pub struct Vertex {
    pub position: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub texture: Vector2<f32>,
}

impl Vertex {
    pub fn new(px: f32, py: f32, pz: f32, nx: f32, ny: f32, nz: f32, tx: f32, ty: f32) -> Self {
        Vertex {
            position: Vector3::new(px, py, pz),
            normal: Vector3::new(nx, ny, nz),
            texture: Vector2::new(tx, ty),
        }
    }
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
