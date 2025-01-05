//! # glwfr (GL Wrapper For Rust)
//!
//! This library provides a high-level wrapper for OpenGL, making it easier to create and manage
//! windows, render 3D scenes, handle input, and play audio in Rust.
//!
//! ## Features
//! - **Graphics**: Window management, OpenGL context creation, texture loading, shader management.
//! - **Scene Management**: Cameras, lights, objects, and transformations.
//! - **Input Handling**: Keyboard and mouse input.
//! - **Audio**: Sound loading and playback.
//!
//! ## Usage
//! Add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! glwfr = "0.4.1"
//! ```
//!
//! ## Example
//! ```rust
//! use glwfr::graphics::{window::Window, gl_wrapper::*};
//! use glwfr::audio::*;
//! use glwfr::gl;
//! use glwfr::cgmath::{Matrix4, Deg, Vector3, Point3, perspective};
//!
//! fn main() -> Result<(), glwfr::custom_errors::Errors> {
//!     // Create a window
//!     let mut window = Window::new(800, 600, "glwfr window example")?;
//!
//!     // Initialize OpenGL
//!     window.init_gl()?;
//!
//!     // Enable depth testing
//!     window.enable_depth_test();
//!
//! 	// Load sound
//! 	let mut audio_system = AudioSystem::new()?;
//! 	audio_system.load_sound("explosion", "explosion.mp3")?;
//!
//!     // Main loop
//!     while !window.should_close() {
//!         // Clear the screen
//!         window.clear(0.0, 0.0, 0.0, 1.0);
//!
//!         // Handle input
//!         if window.is_key_pressed(glwfr::input::Key::Space) {
//!             audio_system.play_sound_once("explosion")?;
//!         }
//!
//!         // Update the window
//!         window.update();
//!     }
//!
//!     Ok(())
//! }
//! ```

pub extern crate cgmath;
pub extern crate gl;
pub mod audio;
pub mod custom_errors;
pub mod graphics;
pub mod input;
pub mod logger;
pub mod scene;
