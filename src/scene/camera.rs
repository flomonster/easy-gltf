use cgmath::*;
use gltf::camera::Projection;

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
        let mut cam = Camera {
            position: Vector3::new(transform[3][0], transform[3][1], transform[3][2]),
            right: Vector3::new(transform[0][0], transform[0][1], transform[0][2]).normalize(),
            up: Vector3::new(transform[1][0], transform[1][1], transform[1][2]).normalize(),
            forward: -1.
                * Vector3::new(transform[2][0], transform[2][1], transform[2][2]).normalize(),
            fov: Rad(90.0),
            zfar: 0.,
            znear: 0.,
        };
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
