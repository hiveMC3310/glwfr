//! # Audio Module
//! This module provides audio functionality.
//!
//! ## Usage
//!
//! ```rust
//! use glwfr::audio::AudioSystem;
//!
//! let audio_system = AudioSystem::new().expect("Failed to initialize audio system");
//! ```

use crate::audio::Sound;
use crate::custom_errors::Errors;
use rodio::{OutputStream, Sink};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Represents the audio system.
pub struct AudioSystem {
    _stream: OutputStream,
    stream_handle: rodio::OutputStreamHandle,
    sounds: HashMap<String, Arc<Mutex<Sound>>>,
}

impl AudioSystem {
    /// Creates a new audio system.
    pub fn new() -> Result<Self, Errors> {
        let (_stream, stream_handle) = OutputStream::try_default()?;
        Ok(Self {
            _stream,
            stream_handle,
            sounds: HashMap::new(),
        })  
    }

    /// Loads a sound from a file and stores it with a given name.
    pub fn load_sound(&mut self, name: &str, file_path: &str) -> Result<(), Errors> {
        let sound = Sound::new(file_path)?;
        self.sounds
            .insert(name.to_string(), Arc::new(Mutex::new(sound)));
        Ok(())
    }

    /// Plays a sound once by its name.
    pub fn play_sound_once(&mut self, name: &str) -> Result<(), Errors> {
        if let Some(sound) = self.sounds.get(name) {
            let mut sound = sound.lock().unwrap();
            let sink = Arc::new(Mutex::new(Sink::try_new(&self.stream_handle)?));
            sound.play_once(&sink)?;
            Ok(())
        } else {
            Err(Errors::SoundNotFoundError(name.to_string()))
        }
    }

    /// Plays a sound in a loop by its name.
    pub fn play_sound_loop(&mut self, name: &str) -> Result<(), Errors> {
        if let Some(sound) = self.sounds.get(name) {
            let mut sound = sound.lock().unwrap();
            let sink = Arc::new(Mutex::new(Sink::try_new(&self.stream_handle)?));
            sound.play_loop(&sink)?;
            Ok(())
        } else {
            Err(Errors::SoundNotFoundError(name.to_string()))
        }
    }

    /// Sets the volume of a specific sound (0.0 to 1.0).
    pub fn set_volume(&self, name: &str, volume: f32) -> Result<(), Errors> {
        if let Some(sound) = self.sounds.get(name) {
            let mut sound = sound.lock().unwrap();
            sound.set_volume(volume)?;
            Ok(())
        } else {
            Err(Errors::SoundNotFoundError(name.to_string()))
        }
    }

    /// Checks if a specific sound is currently playing.
    pub fn is_playing(&self, name: &str) -> Result<bool, Errors> {
        if let Some(sound) = self.sounds.get(name) {
            let mut sound = sound.lock().unwrap();
            Ok(sound.is_playing())
        } else {
            Err(Errors::SoundNotFoundError(name.to_string()))
        }
    }

    /// Checks if a specific sound is paused.
    pub fn is_paused(&self, name: &str) -> Result<bool, Errors> {
        if let Some(sound) = self.sounds.get(name) {
            let sound = sound.lock().unwrap();
            Ok(sound.is_paused())
        } else {
            Err(Errors::SoundNotFoundError(name.to_string()))
        }
    }
}
