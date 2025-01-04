//! # glwfr
//!
//! My library for OpenGL.
//!
//! ## Use
//!
//! ```toml
//! [dependencies]
//! glwfr = "0.4.0"
//! ```
//!
//! ## Example
//!
//! ```rust
//! use glwfr::graphics::{window::Window, gl_wrapper::*};
//! use glwfr::gl;
//! use glwfr::cgmath::{Matrix4, Deg, Vector3, Point3, perspective};
//!
//! fn main() -> Result<(), glwfr::custom_errors::Errors>{
//!     // Create a new window
//!     let mut window = Window::new(800, 600, "glwfr window example")?;
//!
//!     // Initialize OpenGL context
//!     window.init_gl()?;
//!
//!     // Enable depth testing
//!     window.enable_depth_test();
//!
//!     // Main loop
//!     while !window.should_close() {
//!         // Clear the screen and depth buffer
//!         window.clear(0.0, 0.0, 0.0, 1.0);
//!
//!         // Handle input
//!         if window.is_key_pressed(glwfr::input::Key::Escape) {
//!             window.set_should_close(true);
//!         }
//!
//!         // Update window
//!         window.update();
//!     }
//! }
//! ```

pub extern crate cgmath;
pub extern crate gl;
pub mod custom_errors;
pub mod graphics;
pub mod input;
pub mod logger;
pub mod scene;
