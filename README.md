<h1 align="center">
    easy-gltf
</h1>
<p align="center">
   <a href="https://github.com/jordan4ibanez/minetest-gltf/actions">
      <img src="https://github.com/flomonster/minetest-gltf/workflows/Build/badge.svg" alt="github">
   </a>
   <a href="https://crates.io/crates/minetest-gltf">
      <img src="https://img.shields.io/crates/v/minetest-gltf.svg" alt="crates.io">
   </a>
   <a href="https://docs.rs/minetest-gltf">
      <img src="https://docs.rs/minetest-gltf/badge.svg" alt="docs.rs">
   </a>
</p>
<hr>

This crate is intended to load [glTF 2.0](https://www.khronos.org/gltf), a file format designed for efficient transmission of 3D assets.

It's based on the [gltf](https://github.com/gltf-rs/gltf) crate but has an easy to use output.

This crate has been completely modified for the minetest-rust engine.

### Installation

To install it, just add the dependency in your `Cargo.toml`.

```toml
[dependencies]
minetest-gltf="1.1.2"
```

### Usage

For examples of use see the [crate documentation](https://docs.rs/easy-gltf).
