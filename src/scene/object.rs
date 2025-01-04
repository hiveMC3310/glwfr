//! # Object Module
//!
//! This module provides a representation of an object in a 3D scene, including its mesh, transform, and shader program.
//!
//! ## Usage
//!
//! ```rust
//! use glwfr::scene::Object;
//! use glwfr::graphics::gl_wrapper::{Vao, ShaderProgram};
//! use glwfr::cgmath::Matrix4;
//!
//! // Create a new object
//! let vao = Vao::new().unwrap();
//! let shader_program = ShaderProgram::new("vertex.glsl", "fragment.glsl").unwrap();
//! let mut object = Object::new(vao, shader_program);
//!
//! // Set the object's transform
//! object.set_transform(Matrix4::from_translation([1.0, 2.0, 3.0].into()));
//! ```

use crate::graphics::gl_wrapper::{ShaderProgram, Vao};
use cgmath::*;

/// Represents an object in a 3D scene.
pub struct Object {
    /// The mesh of the object, represented as a VAO.
    mesh: Vao,
    /// The transformation matrix of the object.
    pub transform: Matrix4<f32>,
    /// The shader program used to render the object.
    pub shader_program: ShaderProgram,
}

impl Object {
    /// Creates a new object with the specified mesh and shader program.
    ///
    /// # Arguments
    ///
    /// * `mesh` - The mesh of the object, represented as a VAO.
    /// * `shader_program` - The shader program used to render the object.
    ///
    /// # Returns
    ///
    /// A new `Object` instance with the given mesh and shader program,
    /// and an identity transformation matrix.

    pub fn new(mesh: Vao, shader_program: ShaderProgram) -> Self {
        Self {
            mesh,
            transform: Matrix4::identity(),
            shader_program,
        }
    }

    /// Sets the object's transformation matrix.
    ///
    /// # Arguments
    ///
    /// * `transform` - The transformation matrix to set.
    pub fn set_transform(&mut self, transform: Matrix4<f32>) {
        self.transform = transform;
    }

    /// Renders the object using the given view and projection matrices.
    ///
    /// # Arguments
    ///
    /// * `view_matrix` - The view matrix to use for rendering.
    /// * `projection_matrix` - The projection matrix to use for rendering.
    ///
    /// This function binds the object's shader program and sets the "model", "view", and
    /// "projection" uniforms to the object's transformation matrix, the given view matrix,
    /// and the given projection matrix, respectively. It then binds the object's mesh and
    /// renders it using the `gl::DrawElements` function with the `gl::TRIANGLES` primitive type.
    pub fn render(&mut self, view_matrix: Matrix4<f32>, projection_matrix: Matrix4<f32>) {
        self.shader_program.bind();
        self.shader_program
            .set_uniform_matrix4fv("model", &self.transform)
            .unwrap();
        self.shader_program
            .set_uniform_matrix4fv("view", &view_matrix)
            .unwrap();
        self.shader_program
            .set_uniform_matrix4fv("projection", &projection_matrix)
            .unwrap();

        self.mesh.bind();
        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                self.mesh.index_count() as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
        }
    }
}
