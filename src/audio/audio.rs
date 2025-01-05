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
    sinks: HashMap<String, Arc<Mutex<Sink>>>,
}

impl AudioSystem {
    /// Creates a new `AudioSystem` instance with a default output stream and stream handle.
    ///
    /// # Returns
    ///
    /// A `Result` containing an `AudioSystem` instance if successful, or an error of type
    /// `Errors::AudioInitializationError` if there is an error initializing the output stream.
    ///
    /// # Errors
    ///
    /// Returns an `Errors::AudioInitializationError` if the output stream cannot be initialized.

    pub fn new() -> Result<Self, Errors> {
        let (_stream, stream_handle) = OutputStream::try_default()?;
        Ok(Self {
            _stream,
            stream_handle,
            sounds: HashMap::new(),
            sinks: HashMap::new(),
        })
    }

    /// Loads a new sound into the audio system using the provided name and file path.
    ///
    /// # Arguments
    ///
    /// * `name` - The name to associate with the sound.
    /// * `file_path` - The path to the audio file to load.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `()` if successful, or an error of type `Errors` if there is an error
    /// loading the sound.
    ///
    /// # Errors
    ///
    /// Returns an `Errors::SoundLoadError` if the audio file cannot be loaded.
    pub fn load_sound(&mut self, name: &str, file_path: &str) -> Result<(), Errors> {
        let sound = Sound::new(file_path)?;
        self.sounds
            .insert(name.to_string(), Arc::new(Mutex::new(sound)));
        Ok(())
    }

    /// Plays the specified sound once using the audio system.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the sound to play.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `()` if successful, or an error of type `Errors` if there is an error
    /// playing the sound or if the sound is not found.
    ///
    /// # Errors
    ///
    /// Returns an `Errors::SoundNotFoundError` if the sound with the specified name is not found.
    /// Returns an error if there is an issue playing the sound.

    pub fn play_sound_once(&mut self, name: &str) -> Result<(), Errors> {
        if let Some(sound) = self.sounds.get(name) {
            let sound = sound.lock().unwrap();
            let sink = Arc::new(Mutex::new(Sink::try_new(&self.stream_handle)?));
            sound.play_once(&sink)?;
            self.sinks.insert(name.to_string(), sink);
            Ok(())
        } else {
            Err(Errors::SoundNotFoundError(name.to_string()))
        }
    }

    /// Plays the specified sound in an infinite loop using the audio system.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the sound to play.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `()` if successful, or an error of type `Errors` if there is an error
    /// playing the sound or if the sound is not found.
    ///
    /// # Errors
    ///
    /// Returns an `Errors::SoundNotFoundError` if the sound with the specified name is not found.
    /// Returns an error if there is an issue playing the sound.

    pub fn play_sound_loop(&mut self, name: &str) -> Result<(), Errors> {
        if let Some(sound) = self.sounds.get(name) {
            let sound = sound.lock().unwrap();
            let sink = Arc::new(Mutex::new(Sink::try_new(&self.stream_handle)?));
            sound.play_loop(&sink)?;
            self.sinks.insert(name.to_string(), sink);
            Ok(())
        } else {
            Err(Errors::SoundNotFoundError(name.to_string()))
        }
    }

    /// Sets the volume for the specified sound.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the sound for which to set the volume.
    /// * `volume` - The desired volume level, ranging from 0.0 to 1.0.
    ///
    /// # Returns
    ///
    /// A `Result` containing `()` if successful, or an error of type `Errors` if there is an error
    /// setting the volume or if the sound is not found.
    ///
    /// # Errors
    ///
    /// Returns an `Errors::AudioVolumeError` if the volume is not within the range 0.0 to 1.0.
    /// Returns an `Errors::SoundNotFoundError` if the sound with the specified name is not found.

    pub fn set_volume(&self, name: &str, volume: f32) -> Result<(), Errors> {
        if volume < 0.0 || volume > 1.0 {
            return Err(Errors::AudioVolumeError(
                "Volume must be between 0.0 and 1.0".to_string(),
            ));
        }
        if let Some(sink) = self.sinks.get(name) {
            let sink = sink.lock().unwrap();
            sink.set_volume(volume);
            Ok(())
        } else {
            Err(Errors::SoundNotFoundError(name.to_string()))
        }
    }

    /// Sets the volume for all sounds in the audio system.
    ///
    /// # Arguments
    ///
    /// * `volume` - The desired volume level, ranging from 0.0 to 1.0.
    ///
    /// # Returns
    ///
    /// A `Result` containing `()` if successful, or an error of type `Errors` if there is an error
    /// setting the volume.
    ///
    /// # Errors
    ///
    /// Returns an `Errors::AudioVolumeError` if the volume is not within the range 0.0 to 1.0.

    pub fn set_volume_all(&self, volume: f32) -> Result<(), Errors> {
        if volume < 0.0 || volume > 1.0 {
            return Err(Errors::AudioVolumeError(
                "Volume must be between 0.0 and 1.0".to_string(),
            ));
        }
        for sink in self.sinks.values() {
            let sink = sink.lock().unwrap();
            sink.set_volume(volume);
        }
        Ok(())
    }

    /// Pause the sound with the given name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the sound to pause.
    ///
    /// # Returns
    ///
    /// A `Result` containing `()` if successful, or an error of type `Errors` if there is an error
    /// pausing the sound.
    ///
    /// # Errors
    ///
    /// Returns an `Errors::SoundNotFoundError` if the sound with the given name is not found.
    pub fn pause(&self, name: &str) -> Result<(), Errors> {
        if let Some(sink) = self.sinks.get(name) {
            let sink = sink.lock().unwrap();
            sink.pause();
            Ok(())
        } else {
            Err(Errors::SoundNotFoundError(name.to_string()))
        }
    }

    /// Resume the sound with the given name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the sound to resume.
    ///
    /// # Returns
    ///
    /// A `Result` containing `()` if successful, or an error of type `Errors` if there is an error
    /// resuming the sound.
    ///
    /// # Errors
    ///
    /// Returns an `Errors::SoundNotFoundError` if the sound with the given name is not found.
    pub fn resume(&self, name: &str) -> Result<(), Errors> {
        if let Some(sink) = self.sinks.get(name) {
            let sink = sink.lock().unwrap();
            sink.play();
            Ok(())
        } else {
            Err(Errors::SoundNotFoundError(name.to_string()))
        }
    }

    /// Stop the sound with the given name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the sound to stop.
    ///
    /// # Returns
    ///
    /// A `Result` containing `()` if successful, or an error of type `Errors` if there is an error
    /// stopping the sound.
    ///
    /// # Errors
    ///
    /// Returns an `Errors::SoundNotFoundError` if the sound with the given name is not found.
    pub fn stop(&self, name: &str) -> Result<(), Errors> {
        if let Some(sink) = self.sinks.get(name) {
            let sink = sink.lock().unwrap();
            sink.stop();
            Ok(())
        } else {
            Err(Errors::SoundNotFoundError(name.to_string()))
        }
    }

    /// Pause all currently playing sounds in the audio system.
    ///
    /// This method iterates over all sinks in the audio system and pauses each one.
    /// It does not return an error if a sound is not playing, as pausing an already paused
    /// sound is a no-op.

    pub fn pause_all(&self) {
        for sink in self.sinks.values() {
            let sink = sink.lock().unwrap();
            sink.pause();
        }
    }

    /// Resume all paused sounds in the audio system.
    ///
    /// This method iterates over all sinks in the audio system and resumes each one that is paused.
    /// It does not return an error if a sound is not paused, as resuming an already playing
    /// sound is a no-op.
    pub fn resume_all(&self) {
        for sink in self.sinks.values() {
            let sink = sink.lock().unwrap();
            sink.play();
        }
    }

    /// Stop all currently playing sounds in the audio system.
    ///
    /// This method iterates over all sinks in the audio system and stops each one.
    /// It does not return an error if a sound is not playing, as stopping an already stopped
    /// sound is a no-op.
    pub fn stop_all(&self) {
        for sink in self.sinks.values() {
            let sink = sink.lock().unwrap();
            sink.stop();
        }
    }

    /// Checks if a sound is currently playing in the audio system.
    ///
    /// # Arguments
    /// * `name` - The name of the sound to check.
    ///
    /// # Returns
    /// A `Result` containing a boolean indicating whether the sound is playing (`true`) or not (`false`).
    /// If the sound with the given name does not exist, an `Errors::SoundNotFoundError` is returned.
    ///
    /// # Errors
    /// Returns an `Errors::SoundNotFoundError` if the sound with the given name does not exist.
    pub fn is_playing(&self, name: &str) -> Result<bool, Errors> {
        if let Some(sink) = self.sinks.get(name) {
            let sink = sink.lock().unwrap();
            Ok(!sink.is_paused() && !sink.empty())
        } else {
            Err(Errors::SoundNotFoundError(name.to_string()))
        }
    }

    /// Checks if a sound is currently paused in the audio system.
    ///
    /// # Arguments
    /// * `name` - The name of the sound to check.
    ///
    /// # Returns
    /// A `Result` containing a boolean indicating whether the sound is paused (`true`) or not (`false`).
    /// If the sound with the given name does not exist, an `Errors::SoundNotFoundError` is returned.
    ///
    /// # Errors
    /// Returns an `Errors::SoundNotFoundError` if the sound with the given name does not exist.
    pub fn is_paused(&self, name: &str) -> Result<bool, Errors> {
        if let Some(sink) = self.sinks.get(name) {
            let sink = sink.lock().unwrap();
            Ok(sink.is_paused())
        } else {
            Err(Errors::SoundNotFoundError(name.to_string()))
        }
    }
}
