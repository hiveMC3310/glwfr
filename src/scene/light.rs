//! # Light Module
//!
//! This module provides light sources for 3D scenes, supporting point and directional lights.
//!
//! ## Usage
//!
//! ```rust
//! use glwfr::scene::light::{Light, LightType};
//! use glwfr::cgmath::{Point3, Vector3};
//!
//! // Create a point light
//! let point_light = Light::new(
//!     LightType::Point {
//!         position: Point3::new(0.0, 5.0, 0.0),
//!         intensity: 1.0,
//!     },
//!     Vector3::new(1.0, 1.0, 1.0), // Color
//! );
//!
//! // Create a directional light
//! let directional_light = Light::new(
//!     LightType::Directional {
//!         direction: Vector3::new(1.0, -1.0, 0.0),
//!         intensity: 0.8,
//!     },
//!     Vector3::new(1.0, 1.0, 0.8), // Color
//! );
//! ```

use cgmath::*;

/// Represents the type of light source: point or directional.
pub enum LightType {
    /// A point light source with a position and intensity.
    Point {
        position: Point3<f32>,
        intensity: f32,
    },
    /// A directional light source with a direction and intensity.
    Directional {
        direction: Vector3<f32>,
        intensity: f32,
    },
}

/// Represents a light source in a 3D scene.
pub struct Light {
    /// The type of light (point or directional).
    pub light_type: LightType,
    /// The color of the light.
    color: Vector3<f32>,
}

impl Light {
    /// Creates a new light source with the given type and color.
    ///
    /// # Arguments
    ///
    /// * `light_type` - The type of light (point or directional).
    /// * `color` - The color of the light.
    ///
    /// # Returns
    ///
    /// A new `Light` instance with the given type and color.
    pub fn new(light_type: LightType, color: Vector3<f32>) -> Self {
        Self { light_type, color }
    }

    /// Returns the light data including the direction or position, intensity, and color.
    ///
    /// For a point light, this function returns a tuple containing:
    /// - `position`: The position of the point light as a `Vector3<f32>`.
    /// - `intensity`: The intensity of the point light as `f32`.
    /// - `color`: The color of the light as a `Vector3<f32>`.
    ///
    /// For a directional light, this function returns a tuple containing:
    /// - `direction`: The direction of the directional light as a `Vector3<f32>`.
    /// - `intensity`: The intensity of the directional light as `f32`.
    /// - `color`: The color of the light as a `Vector3<f32>`.
    ///
    /// # Returns
    ///
    /// A tuple consisting of a `Vector3<f32>` representing the position or direction,
    /// a `f32` representing the intensity, and a `Vector3<f32>` representing the color.

    pub fn get_light_data(&self) -> (Vector3<f32>, f32, Vector3<f32>) {
        match &self.light_type {
            LightType::Point {
                position,
                intensity,
            } => (position.to_vec(), *intensity, self.color),
            LightType::Directional {
                direction,
                intensity,
            } => (*direction, *intensity, self.color),
        }
    }
}
