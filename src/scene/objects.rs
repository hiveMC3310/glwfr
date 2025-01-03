//! # Objects Module
//!
//! This module provides structures for managing 3D objects, transformations, and scenes in a 3D rendering context.
//!
//! ## Usage
//!
//! ```rust
//! use glwfr::cgmath::{Vector3, Quaternion};
//! use glwfr::objects::{Scene, SceneObject, Transform};
//! use glwfr::graphics::{gl_wrapper::*, texture::Texture};
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
//!     let mut scene_object = SceneObject::new(vao, shader_program, vertex_count);
//!     scene_object.transform = transform;
//!
//!     // Create a scene and add the object
//!     let mut camera = Camera::new(...);
//!     let mut light = Light::new(...);
//!     let mut scene = Scene::new(&mut camera, vec![&mut light]);
//!     scene.add_object(&mut scene_object);
//!
//!     // Draw the scene
//!     scene.draw();
//! }
//! ```

use crate::graphics::{gl_wrapper::*, texture::Texture};
use cgmath::*;

use super::{camera::Camera, light::Light};

/// Represents a 3D scene containing objects, a camera, and lights.
pub struct Scene<'a> {
    /// The objects in the scene.
    pub objects: Vec<&'a mut SceneObject>,
    /// The camera used to view the scene.
    pub camera: &'a mut Camera,
    /// The lights in the scene.
    pub lights: Vec<&'a mut Light>,
}

impl<'a> Scene<'a> {
    /// Creates a new scene with the given camera and lights.
    ///
    /// # Arguments
    ///
    /// * `camera` - The camera used to view the scene.
    /// * `lights` - A vector of lights in the scene.
    ///
    /// # Returns
    ///
    /// A new `Scene` instance.
    pub fn new(camera: &'a mut Camera, lights: Vec<&'a mut Light>) -> Self {
        Self {
            objects: Vec::new(),
            camera,
            lights,
        }
    }

    /// Adds an object to the scene.
    ///
    /// # Arguments
    ///
    /// * `object` - The object to add to the scene.
    pub fn add_object(&mut self, object: &'a mut SceneObject) {
        self.objects.push(object);
    }

    /// Draws all objects in the scene.
    ///
    /// This function binds the VAO, shader program, and texture (if any) for each object
    /// and issues a draw call.
    pub fn draw(&self) {
        for object in &self.objects {
            object.draw();
        }
    }
}

/// Represents a 3D object in a scene.
pub struct SceneObject {
    /// The transformation of the object.
    pub transform: Transform,
    /// The Vertex Array Object (VAO) for the object.
    pub vao: Vao,
    /// The shader program used to render the object.
    pub shader_program: ShaderProgram,
    /// The number of vertices in the object.
    pub vertex_count: usize,
    /// The optional texture applied to the object.
    pub texture: Option<Texture>,
}

impl SceneObject {
    /// Creates a new scene object with the given VAO, shader program, and vertex count.
    ///
    /// # Arguments
    ///
    /// * `vao` - The Vertex Array Object (VAO) for the object.
    /// * `shader_program` - The shader program used to render the object.
    /// * `vertex_count` - The number of vertices in the object.
    ///
    /// # Returns
    ///
    /// A new `SceneObject` instance.
    pub fn new(vao: Vao, shader_program: ShaderProgram, vertex_count: usize) -> Self {
        Self {
            transform: Transform::default(),
            vao,
            shader_program,
            vertex_count,
            texture: None,
        }
    }

    /// Draws the object using its VAO, shader program, and texture.
    ///
    /// This function binds the VAO, shader program, and texture (if any) and issues a draw call.
    pub fn draw(&self) {
        self.vao.bind();
        self.shader_program.bind();
        if let Some(texture) = &self.texture {
            texture.bind(gl::TEXTURE0);
        }
        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertex_count as i32);
        }
    }
}

/// Represents the transformation of an object in 3D space.
#[derive(Copy, Clone)]
pub struct Transform {
    /// The position of the object.
    pub position: Vector3<f32>,
    /// The rotation of the object.
    pub rotation: Quaternion<f32>,
    /// The scale of the object.
    pub scale: Vector3<f32>,
}

impl Default for Transform {
    /// Creates a default transform with position at the origin, no rotation, and uniform scale of 1.
    fn default() -> Self {
        Self {
            position: Vector3::new(0.0, 0.0, 0.0),
            rotation: Quaternion::new(1.0, 0.0, 0.0, 0.0),
            scale: Vector3::new(1.0, 1.0, 1.0),
        }
    }
}

impl Transform {
    /// Creates a new transform with default values.
    ///
    /// # Returns
    ///
    /// A new `Transform` instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Converts the transform into a 4x4 matrix.
    ///
    /// # Returns
    ///
    /// A 4x4 matrix representing the transformation.
    pub fn to_matrix(&self) -> Matrix4<f32> {
        Matrix4::from_translation(self.position)
            * Matrix4::from(self.rotation)
            * Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z)
    }
}
