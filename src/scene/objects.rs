//! # Objects Module
//!
//! This module provides structures and functionality for managing objects in a 3D scene.
//! It includes a `Scene` struct to hold objects, a camera, and lights, as well as a `SceneObject`
//! struct to represent individual objects with transforms, VAOs, and shader programs.
//!
//! ## Usage
//!
//! ```rust
//! use glwfr::scene::objects::{Scene, SceneObject, Transform};
//! use glwfr::graphics::gl_wrapper::{Vao, ShaderProgram};
//! use glwfr::cgmath::{Vector3, Quaternion};
//!
//! fn main() {
//!     // Create a transform for an object
//!     let transform = Transform {
//!         position: Vector3::new(0.0, 0.0, 0.0),
//!         rotation: Quaternion::new(1.0, 0.0, 0.0, 0.0),
//!         scale: Vector3::new(1.0, 1.0, 1.0),
//!     };
//!
//!     // Create a scene object
//!     let scene_object = SceneObject {
//!         transform,
//!         vao: Vao::new(), // Assume Vao::new() creates a valid VAO
//!         shader_program: ShaderProgram::new(), // Assume ShaderProgram::new() creates a valid shader program
//!         vertex_count: 36, // Example vertex count for a cube
//!     };
//!
//!     // Create a scene with the object, a camera, and lights
//!     let scene = Scene {
//!         objects: vec![scene_object],
//!         camera: Camera::default(), // Assume Camera::default() creates a valid camera
//!         lights: vec![], // No lights in this example
//!     };
//! }
//! ```

use crate::graphics::gl_wrapper::*;
use cgmath::*;

use super::{camera::Camera, light::Light};

/// Represents a 3D scene containing objects, a camera, and lights.
///
/// The `Scene` struct is the central container for all elements in a 3D scene.
pub struct Scene {
    /// A list of objects in the scene.
    pub objects: Vec<SceneObject>,
    /// The camera used to view the scene.
    pub camera: Camera,
    /// A list of lights in the scene.
    pub lights: Vec<Light>,
}

/// Represents an object in a 3D scene.
///
/// Each `SceneObject` has a transform, a VAO (Vertex Array Object), a shader program,
/// and a vertex count for rendering.
pub struct SceneObject {
    /// The transform of the object, defining its position, rotation, and scale.
    pub transform: Transform,
    /// The VAO (Vertex Array Object) containing the object's vertex data.
    pub vao: Vao,
    /// The shader program used to render the object.
    pub shader_program: ShaderProgram,
    /// The number of vertices in the object's mesh.
    pub vertex_count: usize,
}

/// Represents the transform of an object in 3D space.
///
/// The `Transform` struct defines the position, rotation, and scale of an object.
pub struct Transform {
    /// The position of the object in world space.
    pub position: Vector3<f32>,
    /// The rotation of the object as a quaternion.
    pub rotation: Quaternion<f32>,
    /// The scale of the object along the X, Y, and Z axes.
    pub scale: Vector3<f32>,
}

impl Transform {
    /// Converts the transform into a 4x4 transformation matrix.
    ///
    /// This matrix can be used to transform vertices from object space to world space.
    ///
    /// # Returns
    ///
    /// A 4x4 matrix representing the combined translation, rotation, and scale of the transform.
    pub fn to_matrix(&self) -> Matrix4<f32> {
        Matrix4::from_translation(self.position)
            * Matrix4::from(self.rotation)
            * Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z)
    }
}
