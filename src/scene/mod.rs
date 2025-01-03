//! # Scene Module
//!
//! This module provides structures and utilities for working with 3D graphics, including cameras, lights, and scene objects.
//!
//! ## Submodules
//!
//! - **`camera`**: Provides a camera implementation for 3D scenes, supporting both perspective and orthographic projections.
//! - **`light`**: Provides structures for managing different types of lights in a 3D scene.
//! - **`objects`**: Provides structures for managing 3D objects, transformations, and scenes.
//!
//! ## Usage
//!
//! ```rust
//! use cgmath::{Deg, Vector3};
//! use glwfr::graphics::{camera::{Camera, CameraProjection}, light::{Light, LightType}, objects::{Scene, SceneObject}};
//!
//! fn main() {
//!     // Create a camera
//!     let mut camera = Camera::new(
//!         Vector3::new(0.0, 0.0, 5.0), // Position
//!         Vector3::new(0.0, 0.0, 0.0), // Target
//!         Vector3::new(0.0, 1.0, 0.0), // Up direction
//!         CameraProjection::Perspective {
//!             fov: Deg(45.0),
//!             aspect_ratio: 16.0 / 9.0,
//!             near: 0.1,
//!             far: 100.0,
//!         },
//!     );
//!
//!     // Create a light
//!     let mut light = Light::new(LightType::Point {
//!         position: Vector3::new(1.0, 2.0, 3.0),
//!         color: Vector3::new(1.0, 1.0, 1.0),
//!         intensity: 1.0,
//!     });
//!
//!     // Create a scene
//!     let mut scene = Scene::new(&mut camera, vec![&mut light]);
//!
//!     // Create a scene object
//!     let mut scene_object = SceneObject::new(vao, shader_program, vertex_count);
//!     scene.add_object(&mut scene_object);
//!
//!     // Draw the scene
//!     scene.draw();
//! }
//! ```

pub mod camera;
pub mod light;
pub mod objects;
