//! # glwfr
//!
//! My library for OpenGL.
//!
//! ## Use
//!
//! ```toml
//! [dependencies]
//! glwfr = "0.2.0"
//! ```
//!
//! ## Example
//!
//! ```rust
//! use glwfr::graphics::{window::Window, gl_wrapper::*};
//! use glwfr::gl;
//! use glwfr::cgmath::{Matrix4, Deg, Vector3, Point3, perspective};
//!
//! fn main() {
//!     let mut window = Window::new(800, 600, "glwfr window example").unwrap();
//!
//! 	window.init_gl();
//!     window.enable_depth_test();
//!
//!     // Main loop
//!     while !window.should_close() {
//!         // Clear the screen
//!         window.clear(0.0, 0.0, 0.0, 1.0);
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
