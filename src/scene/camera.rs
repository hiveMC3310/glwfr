//! # Camera Module
//!
//! This module provides a camera implementation for 3D scenes, supporting both perspective and orthographic projections.
//!
//! ## Usage
//!
//! ```rust
//! use glwfr::cgmath::{Deg, Vector3};
//! use glwfr::camera::{Camera, CameraProjection};
//!
//! fn main() {
//!     // Create a camera with perspective projection
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
//!     // Get the view and projection matrices
//!     let view_matrix = camera.view_matrix();
//!     let projection_matrix = camera.projection_matrix();
//!
//!     // Move the camera to a new position
//!     camera.move_to(Vector3::new(1.0, 2.0, 3.0));
//!
//!     // Change the camera to use orthographic projection
//!     camera.set_orthographic(-1.0, 1.0, -1.0, 1.0, 0.1, 100.0);
//! }
//! ```

use cgmath::*;

/// Represents the type of projection used by the camera.
pub enum CameraProjection {
    /// Perspective projection with a field of view, aspect ratio, and near/far clipping planes.
    Perspective {
        fov: Deg<f32>,     // Field of view in degrees
        aspect_ratio: f32, // Aspect ratio (width / height)
        near: f32,         // Near clipping plane
        far: f32,          // Far clipping plane
    },
    /// Orthographic projection with left, right, bottom, top, and near/far clipping planes.
    Orthographic {
        left: f32,   // Left clipping plane
        right: f32,  // Right clipping plane
        bottom: f32, // Bottom clipping plane
        top: f32,    // Top clipping plane
        near: f32,   // Near clipping plane
        far: f32,    // Far clipping plane
    },
}

/// Represents a camera in a 3D scene.
pub struct Camera {
    /// The position of the camera in world space.
    pub position: Vector3<f32>,
    /// The target point the camera is looking at.
    pub target: Vector3<f32>,
    /// The up direction of the camera.
    pub up: Vector3<f32>,
    /// The projection type (Perspective or Orthographic).
    pub projection: CameraProjection,
}

impl Camera {
    /// Creates a new camera with the given position, target, up direction, and projection.
    ///
    /// # Arguments
    ///
    /// * `position` - The position of the camera in world space.
    /// * `target` - The target point the camera is looking at.
    /// * `up` - The up direction of the camera.
    /// * `projection` - The projection type (Perspective or Orthographic).
    ///
    /// # Returns
    ///
    /// A new `Camera` instance.
    pub fn new(
        position: Vector3<f32>,
        target: Vector3<f32>,
        up: Vector3<f32>,
        projection: CameraProjection,
    ) -> Self {
        Self {
            position,
            target,
            up,
            projection,
        }
    }

    /// Computes the view matrix for the camera.
    ///
    /// The view matrix transforms world coordinates into camera coordinates.
    ///
    /// # Returns
    ///
    /// A 4x4 matrix representing the view transformation.
    pub fn view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(
            Point3::from_vec(self.position),
            Point3::from_vec(self.target),
            self.up,
        )
    }

    /// Computes the projection matrix for the camera.
    ///
    /// The projection matrix transforms camera coordinates into clip space.
    ///
    /// # Returns
    ///
    /// A 4x4 matrix representing the projection transformation.
    pub fn projection_matrix(&self) -> Matrix4<f32> {
        match &self.projection {
            CameraProjection::Perspective {
                fov,
                aspect_ratio,
                near,
                far,
            } => PerspectiveFov {
                fovy: Rad::from(*fov),
                aspect: *aspect_ratio,
                near: *near,
                far: *far,
            }
            .into(),
            CameraProjection::Orthographic {
                left,
                right,
                bottom,
                top,
                near,
                far,
            } => ortho(*left, *right, *bottom, *top, *near, *far),
        }
    }

    /// Moves the camera to a new position.
    ///
    /// # Arguments
    ///
    /// * `position` - The new position of the camera.
    pub fn move_to(&mut self, position: Vector3<f32>) {
        self.position = position;
    }

    /// Makes the camera look at a new target point.
    ///
    /// # Arguments
    ///
    /// * `target` - The new target point.
    pub fn look_at(&mut self, target: Vector3<f32>) {
        self.target = target;
    }

    /// Sets the camera to use orthographic projection.
    ///
    /// # Arguments
    ///
    /// * `left` - The left clipping plane.
    /// * `right` - The right clipping plane.
    /// * `bottom` - The bottom clipping plane.
    /// * `top` - The top clipping plane.
    /// * `near` - The near clipping plane.
    /// * `far` - The far clipping plane.
    pub fn set_orthographic(
        &mut self,
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    ) {
        self.projection = CameraProjection::Orthographic {
            left,
            right,
            bottom,
            top,
            near,
            far,
        };
    }

    /// Sets the camera to use perspective projection.
    ///
    /// # Arguments
    ///
    /// * `fov` - The field of view in degrees.
    /// * `aspect_ratio` - The aspect ratio (width / height).
    /// * `near` - The near clipping plane.
    /// * `far` - The far clipping plane.
    pub fn set_perspective(&mut self, fov: Deg<f32>, aspect_ratio: f32, near: f32, far: f32) {
        self.projection = CameraProjection::Perspective {
            fov,
            aspect_ratio,
            near,
            far,
        };
    }
}
