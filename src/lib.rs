#![deny(missing_docs)]

//! easy-gltf
//!
//!  This crate is intended to load [glTF 2.0](https://www.khronos.org/gltf), a
//!  file format designed for the efficient transmission of 3D assets.
//!
//!  It's base on [gltf](https://github.com/gltf-rs/gltf) crate but has an easy to use output.
//!
//!  # Installation
//!
//!  ```toml
//!  [dependencies]
//!  easy-gltf="0.1.0"
//!  ```
//!
//!  # Example
//!
//! ```
//! # fn run() -> Result<(), Box<dyn std::error::Error>> {
//!      let scenes = easy_gltf::load("tests/cube.glb")?;
//!      for scene in scenes {
//!          println!(
//!              "Cameras: #{}  Lights: #{}  Models: #{}",
//!              scene.cameras.len(),
//!              scene.lights.len(),
//!              scene.models.len()
//!          )
//!      }
//! #     Ok(())
//! # }
//! #
//! #  fn main() {
//! #      let _ = run().expect("runtime error");
//! #  }
//! ```

mod scene;
mod utils;

use gltf::Result;
use std::path::Path;
use utils::GltfData;

pub use scene::*;

/// Load scenes from path to a glTF 2.0.
///
/// Note: You can use this function with either a `Gltf` (standard `glTF`) or `Glb` (binary glTF).
///
/// # Example
///
/// ```
/// # fn run() -> Result<(), Box<dyn std::error::Error>> {
///     let scenes = easy_gltf::load("tests/cube.glb")?;
///     println!("Scenes: #{}", scenes.len()); // Output "Scenes: #1"
///     let scene = &scenes[0]; // Retrieve the first and only scene
///     println!("Cameras: #{}", scene.cameras.len());
///     println!("Lights: #{}", scene.lights.len());
///     println!("Models: #{}", scene.models.len());
/// #   Ok(())
/// # }
/// ```
pub fn load<P>(path: P) -> Result<Vec<Scene>>
where
    P: AsRef<Path>,
{
    let (doc, buffers, images) = gltf::import(path)?;
    let data = GltfData {
        doc,
        buffers,
        images,
    };
    let mut res = vec![];
    let mut collection = Default::default();
    for scene in data.doc.scenes() {
        res.push(Scene::load(scene, &data, &mut collection));
    }
    Ok(res)
}

#[cfg(test)]
mod tests {
    use crate::*;
    use cgmath::*;

    macro_rules! assert_delta {
        ($x:expr, $y:expr, $d:expr) => {
            if !($x - $y < $d || $y - $x < $d) {
                panic!();
            }
        };
    }

    #[test]
    fn check_load() {
        let scenes = load("tests/cube.glb").unwrap();
        assert_eq!(scenes.len(), 1);
        let scene = &scenes[0];
        assert_eq!(scene.cameras.len(), 1);
        assert_eq!(scene.lights.len(), 3);
        assert_eq!(scene.models.len(), 1);
    }

    #[test]
    fn check_camera() {
        let scenes = load("tests/cube.glb").unwrap();
        let scene = &scenes[0];
        let cam = &scene.cameras[0];
        assert!((cam.position - Vector3::new(7.3589, 4.9583, 6.9258)).magnitude() < 0.1);
    }

    #[test]
    fn check_lights() {
        let scenes = load("tests/cube.glb").unwrap();
        let scene = &scenes[0];
        for light in scene.lights.iter() {
            match light {
                Light::Directional {
                    direction,
                    color: _,
                    intensity,
                } => {
                    assert!((direction - Vector3::new(0.6068, -0.7568, -0.2427)).magnitude() < 0.1);
                    assert_delta!(intensity, 542., 0.01);
                }
                Light::Point {
                    position,
                    color: _,
                    intensity,
                } => {
                    assert!((position - Vector3::new(4.0762, 5.9039, -1.0055)).magnitude() < 0.1);
                    assert_delta!(intensity, 1000., 0.01);
                }
                Light::Spot {
                    position,
                    direction,
                    color: _,
                    intensity,
                    inner_cone_angle: _,
                    outer_cone_angle,
                } => {
                    assert!((position - Vector3::new(4.337, 15.541, -8.106)).magnitude() < 0.1);
                    assert!(
                        (direction - Vector3::new(-0.0959, -0.98623, 0.1346)).magnitude() < 0.1
                    );
                    assert_delta!(intensity, 42., 0.01);
                    assert_delta!(outer_cone_angle, 40., 0.01);
                }
            }
        }
    }

    #[test]
    fn check_model() {
        let scenes = load("tests/cube.glb").unwrap();
        let scene = &scenes[0];
        let model = &scene.models[0];
        for t in model.triangles.iter().flatten() {
            let pos = t.position;
            assert!(pos.x > -0.01 && pos.x < 1.01);
            assert!(pos.y > -0.01 && pos.y < 1.01);
            assert!(pos.z > -0.01 && pos.z < 1.01);
        }
    }

    #[test]
    fn check_material() {
        let scenes = load("tests/head.glb").unwrap();
        let scene = &scenes[0];
        let mat = &scene.models[0].material;
        assert!(mat.base_color_texture.is_some());
        assert_eq!(mat.metallic_factor, 0.);
    }

    #[test]
    fn check_invalid_path() {
        assert!(load("tests/invalid.glb").is_err());
    }
}
