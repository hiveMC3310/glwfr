//! # Light Module
//!
//! This module provides structures for managing different types of lights in a 3D scene.
//!
//! ## Usage
//!
//! ```rust
//! use glwfr::cgmath::{Deg, Vector3};
//! use glwfr::light::{Light, LightType};
//!
//! fn main() {
//!     // Create a point light
//!     let mut point_light = Light::new(LightType::Point {
//!         position: Vector3::new(1.0, 2.0, 3.0),
//!         color: Vector3::new(1.0, 1.0, 1.0),
//!         intensity: 1.0,
//!     });
//!
//!     // Change the light's color and intensity
//!     point_light.set_color(Vector3::new(1.0, 0.5, 0.0));
//!     point_light.set_intensity(2.0);
//!
//!     // Convert the light to shader data
//!     let shader_data = point_light.to_shader_data();
//! }
//! ```

use cgmath::*;

/// Represents the type of light in a 3D scene.
pub enum LightType {
    /// A point light with a position, color, and intensity.
    Point {
        position: Vector3<f32>, // Position of the light
        color: Vector3<f32>,    // Color of the light (RGB)
        intensity: f32,         // Intensity of the light
    },
    /// A directional light with a direction, color, and intensity.
    Directional {
        direction: Vector3<f32>, // Direction of the light
        color: Vector3<f32>,     // Color of the light (RGB)
        intensity: f32,          // Intensity of the light
    },
    /// A spot light with a position, direction, color, intensity, and angle.
    Spot {
        position: Vector3<f32>,  // Position of the light
        direction: Vector3<f32>, // Direction of the light
        color: Vector3<f32>,     // Color of the light (RGB)
        intensity: f32,          // Intensity of the light
        angle: Deg<f32>,         // Angle of the spotlight cone
    },
}

/// Represents a light in a 3D scene.
pub struct Light {
    /// The type of light and its parameters.
    pub light_type: LightType,
}

impl Light {
    /// Creates a new light with the specified type.
    ///
    /// # Arguments
    ///
    /// * `light_type` - The type of light and its parameters.
    ///
    /// # Returns
    ///
    /// A new `Light` instance.
    pub fn new(light_type: LightType) -> Self {
        Self { light_type }
    }

    /// Converts the light into shader-friendly data.
    ///
    /// # Returns
    ///
    /// A `LightShaderData` instance containing the light's properties.
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

    /// Sets the color of the light.
    ///
    /// # Arguments
    ///
    /// * `color` - The new color of the light.
    pub fn set_color(&mut self, color: Vector3<f32>) {
        match &mut self.light_type {
            LightType::Point { color: c, .. } => *c = color,
            LightType::Directional { color: c, .. } => *c = color,
            LightType::Spot { color: c, .. } => *c = color,
        }
    }

    /// Sets the intensity of the light.
    ///
    /// # Arguments
    ///
    /// * `intensity` - The new intensity of the light.
    pub fn set_intensity(&mut self, intensity: f32) {
        match &mut self.light_type {
            LightType::Point { intensity: i, .. } => *i = intensity,
            LightType::Directional { intensity: i, .. } => *i = intensity,
            LightType::Spot { intensity: i, .. } => *i = intensity,
        }
    }
}

/// Represents light data in a format suitable for shaders.
#[derive(Clone, Copy)]
pub struct LightShaderData {
    /// The position of the light (for point and spot lights).
    pub position: Vector3<f32>,
    /// The direction of the light (for directional and spot lights).
    pub direction: Vector3<f32>,
    /// The color of the light.
    pub color: Vector3<f32>,
    /// The intensity of the light.
    pub intensity: f32,
    /// The angle of the spotlight cone (for spot lights).
    pub angle: f32,
}
