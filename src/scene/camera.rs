//! # Camera Module
//!
//! This module provides a camera implementation for 3D scenes, supporting both perspective and orthographic projections.
//!
//! ## Usage
//!
//! ```rust
//! use glwfr::scene::camera::{Camera, CameraType};
//! use glwfr::cgmath::{Point3, Vector3, Deg};
//!
//! // Create a perspective camera
//! let camera = Camera::new(
//!     Point3::new(0.0, 0.0, 5.0), // Position
//!     Point3::new(0.0, 0.0, 0.0), // Target
//!     Vector3::new(0.0, 1.0, 0.0), // Up vector
//!     CameraType::Perspective {
//!         fov: Deg(45.0),
//!         aspect: 16.0 / 9.0,
//!         near: 0.1,
//!         far: 100.0,
//!     },
//! );
//!
//! // Get the view and projection matrices
//! let view_matrix = camera.view_matrix();
//! let projection_matrix = camera.projection_matrix();
//! ```

use cgmath::*;

/// Represents the type of camera projection: perspective or orthographic.
pub enum CameraType {
    /// Perspective projection with a field of view, aspect ratio, and near/far clipping planes.
    Perspective {
        fov: Deg<f32>,
        aspect: f32,
        near: f32,
        far: f32,
    },
    /// Orthographic projection with left, right, bottom, top, and near/far clipping planes.
    Orthographic {
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    },
}

/// Represents a camera in a 3D scene.
///
/// The camera defines the view and projection matrices used to render the scene.
pub struct Camera {
    /// The position of the camera in world space.
    pub position: Point3<f32>,
    /// The target point the camera is looking at.
    pub target: Point3<f32>,
    /// The up vector of the camera, defining its orientation.
    pub up: Vector3<f32>,
    /// The type of projection used by the camera (perspective or orthographic).
    camera_type: CameraType,
}

impl Camera {
    /// Creates a new camera with the specified position, target, up vector, and projection type.
    ///
    /// # Arguments
    ///
    /// * `position` - The position of the camera in world space.
    /// * `target` - The target point the camera is looking at.
    /// * `up` - The up vector of the camera, defining its orientation.
    /// * `camera_type` - The type of projection used by the camera (perspective or orthographic).
    ///
    /// # Returns
    ///
    /// A `Camera` instance with the specified parameters.
    pub fn new(
        position: Point3<f32>,
        target: Point3<f32>,
        up: Vector3<f32>,
        camera_type: CameraType,
    ) -> Self {
        Self {
            position,
            target,
            up,
            camera_type,
        }
    }

    /// Returns the view matrix for the camera.
    ///
    /// The view matrix transforms world coordinates into camera coordinates.
    /// The returned matrix is a right-handed matrix, meaning that the camera
    /// is assumed to be looking down the negative z-axis of the world space.
    pub fn view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(self.position, self.target, self.up)
    }

    /// Returns the projection matrix for the camera.
    ///
    /// The projection matrix transforms camera coordinates into normalized device coordinates.
    /// The returned matrix is a right-handed matrix, meaning that the camera is assumed to be
    /// looking down the negative z-axis of the camera space.
    pub fn projection_matrix(&self) -> Matrix4<f32> {
        match &self.camera_type {
            CameraType::Perspective {
                fov,
                aspect,
                near,
                far,
            } => perspective(*fov, *aspect, *near, *far).into(),
            CameraType::Orthographic {
                left,
                right,
                bottom,
                top,
                near,
                far,
            } => ortho(*left, *right, *bottom, *top, *near, *far).into(),
        }
    }
}
