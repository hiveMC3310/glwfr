//! # Scene Module
//!
//! This module provides functionality for managing 3D scenes, including cameras, lights,
//! objects, and transformations.
//!
//! ## Submodules
//! - **camera**: Camera implementation for 3D scenes.
//! - **light**: Light sources for 3D scenes.
//! - **object**: Representation of objects in a 3D scene.
//! - **transform**: Transformations in 3D space.
//!
//! ## Example
//! ```rust
//! use glwfr::scene::{Scene, Camera, Light, Object};
//! use glwfr::graphics::gl_wrapper::{Vao, ShaderProgram};
//! use glwfr::cgmath::{Point3, Vector3, Deg};
//!
//! fn main() -> Result<(), glwfr::custom_errors::Errors> {
//!     // Create a scene
//!     let camera = Camera::new(
//!         Point3::new(0.0, 0.0, 5.0),
//!         Point3::new(0.0, 0.0, 0.0),
//!         Vector3::new(0.0, 1.0, 0.0),
//!         CameraType::Perspective {
//!             fov: Deg(45.0),
//!             aspect: 16.0 / 9.0,
//!             near: 0.1,
//!             far: 100.0,
//!         },
//!     );
//!     let mut scene = Scene::new(camera);
//!
//!     // Add a light to the scene
//!     let light = Light::new(
//!         LightType::Point {
//!             position: Point3::new(0.0, 5.0, 0.0),
//!             intensity: 1.0,
//!         },
//!         Vector3::new(1.0, 1.0, 1.0),
//!     );
//!     scene.add_light(light);
//!
//!     // Add an object to the scene
//!     let vao = Vao::new()?;
//!     let shader_program = ShaderProgram::new("vertex.glsl", "fragment.glsl")?;
//!     let object = Object::new(vao, shader_program);
//!     scene.add_object(object);
//!
//!     // Render the scene
//!     scene.render();
//!
//!     Ok(())
//! }
//! ```

pub mod camera;
pub mod light;
pub mod object;
pub mod scene;
pub mod transform;

pub use camera::*;
pub use light::*;
pub use object::*;
pub use scene::*;
pub use transform::*;
