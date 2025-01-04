//! # Scene Module
//!
//! This module provides the core components for managing a 3D scene, including cameras, lights, objects, and the scene itself.
//!
//! ## Modules
//!
//! - `camera`: Provides a camera implementation for 3D scenes, supporting both perspective and orthographic projections.
//! - `light`: Provides light sources for 3D scenes, including point and directional lights.
//! - `object`: Represents objects in a 3D scene, including their mesh, transform, and shader program.
//! - `scene`: Manages a collection of cameras, lights, and objects to represent a complete 3D scene.

pub mod camera;
pub mod light;
pub mod object;
pub mod scene;

pub use camera::*;
pub use light::*;
pub use object::*;
pub use scene::*;
