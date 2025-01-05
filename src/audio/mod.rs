//! # Audio Module
//!
//! This module provides functionality for loading and playing audio in your application.
//!
//! ## Submodules
//! - **audio**: The main audio system for managing sounds.
//! - **sound**: Represents a sound that can be played.
//!
//! ## Example
//! ```rust
//! use glwfr::audio::{AudioSystem, Sound};
//!
//! fn main() -> Result<(), glwfr::custom_errors::Errors> {
//!     // Create an audio system
//!     let mut audio_system = AudioSystem::new()?;
//!
//!     // Load a sound
//!     audio_system.load_sound("background", "path/to/sound.wav")?;
//!
//!     // Play the sound in a loop
//!     audio_system.play_sound_loop("background")?;
//!
//!     Ok(())
//! }
//! ```

pub mod audio;
pub mod sound;

pub use audio::*;
pub use sound::*;
