#![deny(missing_docs)]

//! This crate is intended to load [glTF 2.0](https://www.khronos.org/gltf), a
//! file format designed for the efficient transmission of 3D assets.
//!
//! It's base on [gltf](https://github.com/gltf-rs/gltf) crate but has an easy to use output.
//!
//! # Installation
//!
//! ```toml
//! [dependencies]
//! easy-gltf="1.1.1"
//! ```
//!
//! # Example
//!
//! ```
//! let scenes = minetest_gltf::load("tests/cube.glb", true).expect("Failed to load glTF");
//! for scene in scenes {
//!     println!(
//!         "Cameras: #{}  Lights: #{}  Models: #{}",
//!         scene.cameras.len(),
//!         scene.lights.len(),
//!         scene.models.len()
//!     )
//! }
//! ```

mod scene;
mod utils;

use gltf::Gltf;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
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
/// let scenes = minetest_gltf::load("tests/cube.glb", true).expect("Failed to load glTF");
/// println!("Scenes: #{}", scenes.len()); // Output "Scenes: #1"
/// let scene = &scenes[0]; // Retrieve the first and only scene
/// println!("Cameras: #{}", scene.cameras.len());
/// println!("Lights: #{}", scene.lights.len());
/// println!("Models: #{}", scene.models.len());
/// ```
pub fn load(path: &str, load_images: bool) -> Result<Vec<Scene>, Box<dyn Error + Send + Sync>> {
  // Run gltf

  // We need the base path for the GLTF lib. We want to choose if we load textures.
  let base = Path::new(path).parent().unwrap_or_else(|| Path::new("./"));

  // The buffer we're going to read the model into.
  let model_reader = read_path_to_buf_read(path)?;

  // Now we need to get the "Document" from the GLTF lib.
  let gltf_data = match Gltf::from_reader(model_reader) {
    Ok(data) => data,
    Err(e) => panic!("{}", e),
  };

  // We're going to do some manual integration here.

  // We always want the buffer data. We have to clone this, it's basically ripping out ownership from our hands.
  let buffers = gltf::import_buffers(&gltf_data.clone(), Some(base), gltf_data.blob.clone())?;

  // But we only want the image data if the programmer wants it.
  let images = match load_images {
    true => Some(gltf::import_images(
      &gltf_data.clone(),
      Some(base),
      &buffers,
    )?),
    false => None,
  };

  // Init data and collection useful for conversion
  let mut data = GltfData::new(buffers, images, &path);

  // Convert gltf -> minetest_gltf
  let mut res = vec![];
  for scene in gltf_data.scenes() {
    res.push(Scene::load(scene, &mut data, load_images));
  }
  Ok(res)
}

///
/// Automatically parse a file path into a BufReader<File>.
///
fn read_path_to_buf_read(path: &str) -> Result<BufReader<File>, String> {
  match File::open(path) {
    Ok(file) => Ok(BufReader::new(file)),
    Err(e) => Err(format!("Path to BufReader failure. {}", e)),
  }
}

#[cfg(test)]
mod tests {
  use crate::model::Mode;
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
  fn load_snowman() {
    let scenes = match load("tests/snowman.gltf", false) {
      Ok(scenes) => {
        println!("Snowman loaded!");
        scenes
      }
      Err(e) => panic!("Snowman failed: {}", e),
    };

    assert_eq!(scenes.first().unwrap().models.len(), 5);
  }

  #[test]
  fn check_cube_glb() {
    let scenes = load("tests/cube.glb", true).unwrap();
    assert_eq!(scenes.len(), 1);
    let scene = &scenes[0];
    assert_eq!(scene.cameras.len(), 1);
    assert_eq!(scene.lights.len(), 3);
    assert_eq!(scene.models.len(), 1);
  }

  #[test]
  fn check_different_meshes() {
    let scenes = load("tests/complete.glb", true).unwrap();
    assert_eq!(scenes.len(), 1);
    let scene = &scenes[0];
    for model in scene.models.iter() {
      match model.mode() {
        Mode::Triangles | Mode::TriangleFan | Mode::TriangleStrip => {
          assert!(model.triangles().is_ok());
        }
        Mode::Lines | Mode::LineLoop | Mode::LineStrip => {
          assert!(model.lines().is_ok());
        }
        Mode::Points => {
          assert!(model.points().is_ok());
        }
      }
    }
  }

  #[test]
  fn check_cube_gltf() {
    let _ = load("tests/cube_classic.gltf", true).unwrap();
  }

  #[test]
  fn check_default_texture() {
    let _ = load("tests/box_sparse.glb", true).unwrap();
  }

  #[test]
  fn check_camera() {
    let scenes = load("tests/cube.glb", true).unwrap();
    let scene = &scenes[0];
    let cam = &scene.cameras[0];
    assert!((cam.position() - Vector3::new(7.3589, 4.9583, 6.9258)).magnitude() < 0.1);
  }

  #[test]
  fn check_lights() {
    let scenes = load("tests/cube.glb", true).unwrap();
    let scene = &scenes[0];
    for light in scene.lights.iter() {
      match light {
        Light::Directional {
          direction,
          color: _,
          intensity,
          ..
        } => {
          assert!((direction - Vector3::new(0.6068, -0.7568, -0.2427)).magnitude() < 0.1);
          assert_delta!(intensity, 542., 0.01);
        }
        Light::Point {
          position,
          color: _,
          intensity,
          ..
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
          ..
        } => {
          assert!((position - Vector3::new(4.337, 15.541, -8.106)).magnitude() < 0.1);
          assert!((direction - Vector3::new(-0.0959, -0.98623, 0.1346)).magnitude() < 0.1);
          assert_delta!(intensity, 42., 0.01);
          assert_delta!(outer_cone_angle, 40., 0.01);
        }
      }
    }
  }

  #[test]
  fn check_model() {
    let scenes = load("tests/cube.glb", true).unwrap();
    let scene = &scenes[0];
    let model = &scene.models[0];
    assert!(model.has_normals());
    assert!(model.has_tex_coords());
    assert!(model.has_tangents());
    for t in model.triangles().unwrap().iter().flatten() {
      let pos = t.position;
      assert!(pos.x > -0.01 && pos.x < 1.01);
      assert!(pos.y > -0.01 && pos.y < 1.01);
      assert!(pos.z > -0.01 && pos.z < 1.01);

      // Check that the tangent w component is 1 or -1
      assert_eq!(t.tangent.w.abs(), 1.);
    }
  }

  #[test]
  fn check_material() {
    let scenes = load("tests/head.glb", true).unwrap();
    let scene = &scenes[0];
    let mat = &scene.models[0].material.as_ref().unwrap();
    assert!(mat.pbr.base_color_texture.is_some());
    assert_eq!(mat.pbr.metallic_factor, 0.);
  }

  #[test]
  fn check_invalid_path() {
    assert!(load("tests/invalid.glb", true).is_err());
  }
}
