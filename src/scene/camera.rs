use cgmath::*;
use gltf::camera::Projection;

/// Contains camera properties.
#[derive(Clone, Debug)]
pub struct Camera {
    /// Position of the camera
    pub position: Vector3<f32>,

    /// Right vector of the camera
    pub right: Vector3<f32>,

    /// Up vector of the camera
    pub up: Vector3<f32>,

    /// Forward vector of the camera (backside direction)
    pub forward: Vector3<f32>,

    /// Angle in degree of field of view
    pub fov: Rad<f32>,

    /// The distance to the far clipping plane.
    pub zfar: f32,

    /// The distance to the near clipping plane.
    pub znear: f32,
}

impl Camera {
    pub(crate) fn load(gltf_cam: gltf::Camera, transform: &Matrix4<f32>) -> Self {
        let mut cam = Self::default();
        cam.position = Vector3::new(transform[3][0], transform[3][1], transform[3][2]);
        cam.right = Vector3::new(transform[0][0], transform[0][1], transform[0][2]).normalize();
        cam.up = Vector3::new(transform[1][0], transform[1][1], transform[1][2]).normalize();
        cam.forward =
            -1. * Vector3::new(transform[2][0], transform[2][1], transform[2][2]).normalize();
        match gltf_cam.projection() {
            Projection::Orthographic(ortho) => {
                cam.zfar = ortho.zfar();
                cam.znear = ortho.znear();
            }
            Projection::Perspective(pers) => {
                cam.zfar = pers.zfar().unwrap_or(f32::INFINITY);
                cam.znear = pers.znear();
                cam.fov = Rad(pers.yfov());
            }
        };
        cam
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            position: Vector3::zero(),
            right: Vector3::new(1., 0., 0.),
            up: Vector3::new(0., 1., 0.),
            forward: Vector3::new(0., 0., -1.),
            fov: Rad(0.399),
            zfar: f32::INFINITY,
            znear: 0.,
        }
    }
}
