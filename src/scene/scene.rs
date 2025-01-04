//! # Scene Module
//!
//! This module provides a representation of a 3D scene, including a camera, lights, and objects.
//!
//! ## Usage
//!
//! ```rust
//! use glwfr::scene::{Scene, Camera, Light, Object};
//! use glwfr::graphics::gl_wrapper::{Vao, ShaderProgram};
//! use glwfr::cgmath::{Point3, Vector3, Deg};
//!
//! // Create a scene
//! let camera = Camera::new(
//!     Point3::new(0.0, 0.0, 5.0),
//!     Point3::new(0.0, 0.0, 0.0),
//!     Vector3::new(0.0, 1.0, 0.0),
//!     CameraType::Perspective {
//!         fov: Deg(45.0),
//!         aspect: 16.0 / 9.0,
//!         near: 0.1,
//!         far: 100.0,
//!     },
//! );
//! let mut scene = Scene::new(camera);
//!
//! // Add a light to the scene
//! let light = Light::new(
//!     LightType::Point {
//!         position: Point3::new(0.0, 5.0, 0.0),
//!         intensity: 1.0,
//!     },
//!     Vector3::new(1.0, 1.0, 1.0),
//! );
//! scene.add_light(light);
//!
//! // Add an object to the scene
//! let vao = Vao::new().unwrap();
//! let shader_program = ShaderProgram::new("vertex.glsl", "fragment.glsl").unwrap();
//! let object = Object::new(vao, shader_program);
//! scene.add_object(object);
//!
//! // Render the scene
//! scene.render();
//!
use super::{Camera, Light, Object};

/// Represents a 3D scene containing a camera, lights, and objects.
pub struct Scene {
    /// The camera used to view the scene.
    camera: Camera,
    /// The lights in the scene.
    lights: Vec<Light>,
    /// The objects in the scene.
    objects: Vec<Object>,
}

impl Scene {
    /// Creates a new scene with the specified camera and no lights or objects.
    ///
    /// # Arguments
    ///
    /// * `camera` - The camera used to view the scene.
    ///
    /// # Returns
    ///
    /// A `Scene` with the specified camera and empty lists of lights and objects.
    pub fn new(camera: Camera) -> Self {
        Self {
            camera,
            lights: Vec::new(),
            objects: Vec::new(),
        }
    }

    /// Adds a light to the scene.
    ///
    /// # Arguments
    ///
    /// * `light` - The light to add to the scene.
    ///
    /// # Returns
    ///
    /// None
    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    /// Returns a mutable reference to the vector of lights in the scene.
    ///
    /// # Returns
    ///
    /// A mutable reference to the vector of lights in the scene.
    pub fn get_mut_lights(&mut self) -> &mut Vec<Light> {
        &mut self.lights
    }

    /// Returns a mutable reference to the camera in the scene.
    ///
    /// # Returns
    ///
    /// A mutable reference to the camera in the scene.
    pub fn get_mut_camera(&mut self) -> &mut Camera {
        &mut self.camera
    }

    /// Returns a mutable reference to the object at the specified index in the scene, or None if the index is out of bounds.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the object to return.
    ///
    /// # Returns
    ///
    /// A mutable reference to the object at the specified index, or None if the index is out of bounds.
    pub fn get_mut_object(&mut self, index: usize) -> Option<&mut Object> {
        self.objects.get_mut(index)
    }

    /// Adds an object to the scene.
    ///
    /// # Arguments
    ///
    /// * `object` - The object to add to the scene.
    ///
    /// # Returns
    ///
    /// None
    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }

    /// Renders all objects in the scene using the current camera's view and projection matrices.
    ///
    /// # Description
    ///
    /// This function iterates over all objects in the scene and calls their `render` method with the
    /// current view and projection matrices for the camera. This allows each object to render itself
    /// using its own mesh and shader program.
    ///
    /// # Note
    ///
    /// This function does not clear the OpenGL context or swap the front and back buffers; it is
    /// expected that the caller will handle these tasks.
    pub fn render(&mut self) {
        let view_matrix = self.camera.view_matrix();
        let projection_matrix = self.camera.projection_matrix();

        for object in &mut self.objects {
            object.render(view_matrix, projection_matrix);
        }
    }
}
