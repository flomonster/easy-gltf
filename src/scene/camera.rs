use cgmath::*;
use gltf::camera::Projection as GltfProjection;

/// Contains camera properties.
#[derive(Clone, Debug)]
pub struct Camera {
  #[cfg(feature = "names")]
  /// Camera name. Requires the `names` feature.
  pub name: Option<String>,

  #[cfg(feature = "extras")]
  /// Scene extra data. Requires the `extras` feature.
  pub extras: gltf::json::extras::Extras,

  /// Transform matrix (also called world to camera matrix)
  pub transform: Matrix4<f32>,

  /// Projection type and specific parameters
  pub projection: Projection,

  /// The distance to the far clipping plane.
  ///
  /// For perspective projection, this may be infinite.
  pub zfar: f32,

  /// The distance to the near clipping plane.
  pub znear: f32,
}

/// Camera projections
#[derive(Debug, Clone)]
pub enum Projection {
  /// Perspective projection
  Perspective {
    /// Y-axis FOV, in radians
    yfov: Rad<f32>,
    /// Aspect ratio, if specified
    aspect_ratio: Option<f32>,
  },
  /// Orthographic projection
  Orthographic {
    /// Projection scale
    scale: Vector2<f32>,
  },
}
impl Default for Projection {
  fn default() -> Self {
    Self::Perspective {
      yfov: Rad(0.399),
      aspect_ratio: None,
    }
  }
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
  /// # use minetest_gltf::Camera;
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
    let mut cam = Self {
      transform: *transform,
      ..Default::default()
    };

    #[cfg(feature = "names")]
    {
      cam.name = gltf_cam.name().map(String::from);
    }
    #[cfg(feature = "extras")]
    {
      cam.extras = gltf_cam.extras().clone();
    }

    match gltf_cam.projection() {
      GltfProjection::Orthographic(ortho) => {
        cam.projection = Projection::Orthographic {
          scale: Vector2::new(ortho.xmag(), ortho.ymag()),
        };
        cam.zfar = ortho.zfar();
        cam.znear = ortho.znear();
      }
      GltfProjection::Perspective(pers) => {
        cam.projection = Projection::Perspective {
          yfov: Rad(pers.yfov()),
          aspect_ratio: pers.aspect_ratio(),
        };
        cam.zfar = pers.zfar().unwrap_or(f32::INFINITY);
        cam.znear = pers.znear();
      }
    };
    cam
  }
}

impl Default for Camera {
  fn default() -> Self {
    Camera {
      #[cfg(feature = "names")]
      name: None,
      #[cfg(feature = "extras")]
      extras: None,
      transform: Zero::zero(),
      projection: Projection::default(),
      zfar: f32::INFINITY,
      znear: 0.,
    }
  }
}
