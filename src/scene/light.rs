//! # Light Module
//!
//! This module provides a light implementation for 3D scenes, supporting point, directional, and spot lights.
//!
//! ## Usage
//!
//! ```rust
//! use glwfr::scene::light::{Light, LightType};
//! use glwfr::cgmath::{Deg, Vector3};
//!
//! fn main() {
//!     // Create a point light
//!     let point_light = Light {
//!         light_type: LightType::Point {
//!             position: Vector3::new(0.0, 5.0, 0.0),
//!             color: Vector3::new(1.0, 1.0, 1.0),
//!             intensity: 1.0,
//!         },
//!     };
//!
//!     // Create a directional light
//!     let directional_light = Light {
//!         light_type: LightType::Directional {
//!             direction: Vector3::new(1.0, -1.0, 0.0),
//!             color: Vector3::new(1.0, 1.0, 0.8),
//!             intensity: 0.7,
//!         },
//!     };
//!
//!     // Create a spot light
//!     let spot_light = Light {
//!         light_type: LightType::Spot {
//!             position: Vector3::new(0.0, 5.0, 0.0),
//!             direction: Vector3::new(0.0, -1.0, 0.0),
//!             color: Vector3::new(1.0, 0.5, 0.5),
//!             intensity: 1.0,
//!             angle: Deg(30.0),
//!         },
//!     };
//!
//!     // Convert lights to shader data
//!     let point_light_data = point_light.to_shader_data();
//!     let directional_light_data = directional_light.to_shader_data();
//!     let spot_light_data = spot_light.to_shader_data();
//! }
//! ```

use cgmath::*;

/// Represents the type of light in a 3D scene.
///
/// This enum supports three types of lights:
/// - **Point**: A light that emits light in all directions from a single point.
/// - **Directional**: A light that emits light in a specific direction, like sunlight.
/// - **Spot**: A light that emits light in a cone, like a flashlight.
pub enum LightType {
    /// A point light with a position, color, and intensity.
    Point {
        /// The position of the light in world space.
        position: Vector3<f32>,
        /// The color of the light.
        color: Vector3<f32>,
        /// The intensity of the light.
        intensity: f32,
    },
    /// A directional light with a direction, color, and intensity.
    Directional {
        /// The direction of the light.
        direction: Vector3<f32>,
        /// The color of the light.
        color: Vector3<f32>,
        /// The intensity of the light.
        intensity: f32,
    },
    /// A spot light with a position, direction, color, intensity, and cone angle.
    Spot {
        /// The position of the light in world space.
        position: Vector3<f32>,
        /// The direction of the light.
        direction: Vector3<f32>,
        /// The color of the light.
        color: Vector3<f32>,
        /// The intensity of the light.
        intensity: f32,
        /// The angle of the light cone in degrees.
        angle: Deg<f32>,
    },
}

/// Represents a light in a 3D scene.
///
/// The light can be of type `Point`, `Directional`, or `Spot`.
pub struct Light {
    /// The type of light and its parameters.
    pub light_type: LightType,
}

impl Light {
    /// Converts the light into a format suitable for use in shaders.
    ///
    /// This method extracts the relevant data from the light and returns it as a `LightShaderData` struct.
    ///
    /// # Returns
    ///
    /// A `LightShaderData` struct containing the light's position, direction, color, intensity, and angle.
    pub fn to_shader_data(&self) -> LightShaderData {
        match &self.light_type {
            LightType::Point {
                position,
                color,
                intensity,
            } => LightShaderData {
                position: *position,
                direction: Vector3::new(0.0, 0.0, 0.0), // Not used for point lights
                color: *color,
                intensity: *intensity,
                angle: 0.0, // Not used for point lights
            },
            LightType::Directional {
                direction,
                color,
                intensity,
            } => LightShaderData {
                position: Vector3::new(0.0, 0.0, 0.0), // Not used for directional lights
                direction: *direction,
                color: *color,
                intensity: *intensity,
                angle: 0.0, // Not used for directional lights
            },
            LightType::Spot {
                position,
                direction,
                color,
                intensity,
                angle,
            } => LightShaderData {
                position: *position,
                direction: *direction,
                color: *color,
                intensity: *intensity,
                angle: angle.0.to_radians(),
            },
        }
    }
}

/// Represents the data for a light that can be passed to a shader.
///
/// This struct contains the position, direction, color, intensity, and angle of the light.
#[derive(Clone, Copy)]
pub struct LightShaderData {
    /// The position of the light in world space.
    pub position: Vector3<f32>,
    /// The direction of the light.
    pub direction: Vector3<f32>,
    /// The color of the light.
    pub color: Vector3<f32>,
    /// The intensity of the light.
    pub intensity: f32,
    /// The angle of the light cone in radians (used for spot lights).
    pub angle: f32,
}
