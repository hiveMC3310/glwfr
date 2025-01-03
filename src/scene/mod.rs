//! # Scene Module
//!
//! This module provides functionality for managing 3D scenes, including cameras, lights, and objects.
//!
//! ## Submodules
//!
//! - **camera**: Provides a camera implementation for viewing 3D scenes.
//! - **light**: Provides light sources for illuminating 3D scenes.
//! - **objects**: Provides structures for managing objects in a 3D scene.
//!
//! ## Usage
//!
//! ```rust
//! use glwfr::scene::{camera::Camera, light::Light, objects::{Scene, SceneObject, Transform}};
//! use glwfrcgmath::{Vector3, Quaternion};
//! use glwfr::graphics::gl_wrapper::{Vao, ShaderProgram};
//!
//! fn main() {
//!     // Create a camera
//!     let camera = Camera {
//!         position: Vector3::new(0.0, 0.0, 5.0),
//!         target: Vector3::new(0.0, 0.0, 0.0),
//!         up: Vector3::new(0.0, 1.0, 0.0),
//!         projection: camera::CameraProjection::Perspective {
//!             fov: cgmath::Deg(45.0),
//!             aspect_ratio: 16.0 / 9.0,
//!             near: 0.1,
//!             far: 100.0,
//!         },
//!     };
//!
//!     // Create a light
//!     let light = Light {
//!         light_type: light::LightType::Point {
//!             position: Vector3::new(0.0, 5.0, 0.0),
//!             color: Vector3::new(1.0, 1.0, 1.0),
//!             intensity: 1.0,
//!         },
//!     };
//!
//!     // Create a scene object
//!     let scene_object = SceneObject {
//!         transform: Transform {
//!             position: Vector3::new(0.0, 0.0, 0.0),
//!             rotation: Quaternion::new(1.0, 0.0, 0.0, 0.0),
//!             scale: Vector3::new(1.0, 1.0, 1.0),
//!         },
//!         vao: Vao::new(), // Assume Vao::new() creates a valid VAO
//!         shader_program: ShaderProgram::new(), // Assume ShaderProgram::new() creates a valid shader program
//!         vertex_count: 36, // Example vertex count for a cube
//!     };
//!
//!     // Create a scene
//!     let scene = Scene {
//!         objects: vec![scene_object],
//!         camera,
//!         lights: vec![light],
//!     };
//! }
//! ```

pub mod camera;
pub mod light;
pub mod objects;
