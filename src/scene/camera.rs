use cgmath::*;
use gltf::camera::Projection;

/// Contains camera properties.
#[derive(Clone, Debug)]
pub struct Camera {
    /// Transform matrix (also called world to camera matrix)
    pub transform: Matrix4<f32>,

    /// Angle in degree of field of view
    pub fov: Rad<f32>,

    /// The distance to the far clipping plane.
    pub zfar: f32,

    /// The distance to the near clipping plane.
    pub znear: f32,
}

impl Camera {
    /// Position of the camera.
    pub fn position(&self) -> Vector3<f32> {
        Vector3::new(
            self.transform[3][0],
            self.transform[3][1],
            self.transform[3][2],
        )
    }

    /// Right vector of the camera.
    pub fn right(&self) -> Vector3<f32> {
        Vector3::new(
            self.transform[0][0],
            self.transform[0][1],
            self.transform[0][2],
        )
        .normalize()
    }

    /// Up vector of the camera.
    pub fn up(&self) -> Vector3<f32> {
        Vector3::new(
            self.transform[1][0],
            self.transform[1][1],
            self.transform[1][2],
        )
        .normalize()
    }

    /// Forward vector of the camera (backside direction).
    pub fn forward(&self) -> Vector3<f32> {
        Vector3::new(
            self.transform[2][0],
            self.transform[2][1],
            self.transform[2][2],
        )
        .normalize()
    }

    /// Apply the transformation matrix on a vector.
    ///
    /// # Example
    /// ```
    /// # use easy_gltf::Camera;
    /// # use cgmath::*;
    /// # let cam = Camera::default();
    /// let ray_dir = Vector3::new(1., 0., 0.);
    /// let ray_dir = cam.apply_transform_vector(&ray_dir);
    /// ```
    pub fn apply_transform_vector(&self, pos: &Vector3<f32>) -> Vector3<f32> {
        let pos = Vector4::new(pos[0], pos[1], pos[2], 0.);
        (self.transform * pos).truncate()
    }

    pub(crate) fn load(gltf_cam: gltf::Camera, transform: &Matrix4<f32>) -> Self {
        let mut cam = Self::default();
        cam.transform = transform.clone();
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
            transform: Zero::zero(),
            fov: Rad(0.399),
            zfar: f32::INFINITY,
            znear: 0.,
        }
    }
}
