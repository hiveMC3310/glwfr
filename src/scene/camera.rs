//! # Camera Module
//!
//! This module provides a camera implementation for 3D scenes, supporting both perspective and orthographic projections.
//!
//! ## Usage
//!
//! ```rust
//! use glwfr::scene::camera::{Camera, CameraProjection};
//! use glwfrcgmath::{Deg, Vector3};
//!
//! fn main() {
//!     // Create a perspective camera
//!     let camera = Camera {
//!         position: Vector3::new(0.0, 0.0, 5.0),
//!         target: Vector3::new(0.0, 0.0, 0.0),
//!         up: Vector3::new(0.0, 1.0, 0.0),
//!         projection: CameraProjection::Perspective {
//!             fov: Deg(45.0),
//!             aspect_ratio: 16.0 / 9.0,
//!             near: 0.1,
//!             far: 100.0,
//!         },
//!     };
//!
//!     // Get the view and projection matrices
//!     let view_matrix = camera.view_matrix();
//!     let projection_matrix = camera.projection_matrix();
//! }
//! ```

use cgmath::*;

/// Represents the type of projection used by the camera.
///
/// This enum supports two types of projections:
/// - **Perspective**: Simulates a realistic view with depth perception.
/// - **Orthographic**: Simulates a flat, 2D-like view without depth perception.
pub enum CameraProjection {
    /// Perspective projection with a field of view, aspect ratio, and near/far clipping planes.
    Perspective {
        /// The field of view in degrees.
        fov: Deg<f32>,
        /// The aspect ratio (width / height) of the viewport.
        aspect_ratio: f32,
        /// The distance to the near clipping plane.
        near: f32,
        /// The distance to the far clipping plane.
        far: f32,
    },
    /// Orthographic projection with left, right, bottom, top, and near/far clipping planes.
    Orthographic {
        /// The left clipping plane.
        left: f32,
        /// The right clipping plane.
        right: f32,
        /// The bottom clipping plane.
        bottom: f32,
        /// The top clipping plane.
        top: f32,
        /// The distance to the near clipping plane.
        near: f32,
        /// The distance to the far clipping plane.
        far: f32,
    },
}

/// Represents a camera in a 3D scene.
///
/// The camera defines the view and projection matrices used to render the scene.
pub struct Camera {
    /// The position of the camera in world space.
    pub position: Vector3<f32>,
    /// The target point the camera is looking at.
    pub target: Vector3<f32>,
    /// The up direction of the camera.
    pub up: Vector3<f32>,
    /// The projection type and parameters for the camera.
    pub projection: CameraProjection,
}

impl Camera {
    /// Computes the view matrix for the camera.
    ///
    /// The view matrix transforms world coordinates into camera (eye) coordinates.
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
    /// The projection matrix transforms camera coordinates into clip coordinates.
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
}
