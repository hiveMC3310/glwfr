//! # Sound Module
//! Represents a sound that can be played using the audio system.
//!
//! Sounds are loaded from audio files and can be played once or in a loop.
//! The audio data is stored in a buffer for efficient playback.
//!
//! # Supported Formats
//! - WAV
//! - MP3
//! - OGG
//!
//! # Example
//! ```rust
//! use glwfr::audio::sound::Sound;
//!
//! let sound = Sound::new("path/to/sound.wav").expect("Failed to load sound");
//! ```

use crate::custom_errors::Errors;
use rodio::{Decoder, Sink, Source};
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};

/// Represents a sound that can be played.
pub struct Sound {
    data: Vec<u16>,
}

impl Sound {
    /// Creates a new `Sound` instance from a file path.
    ///
    /// # Arguments
    ///
    /// * `file_path` - The path to the audio file to load.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `Sound` instance if successful, or an error of type `Errors` if there
    /// is an error opening the file or decoding the audio data.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be opened or if the audio data cannot be decoded.

    pub fn new(file_path: &str) -> Result<Self, Errors> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let decoder = Decoder::new(reader)?;
        let data: Vec<u16> = decoder.convert_samples().collect();
        Ok(Self { data })
    }

    /// Plays the sound once using the provided sink.
    ///
    /// # Arguments
    ///
    /// * `sink` - The sink to play the sound on.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `()` if successful, or an error of type `Errors` if there is an error
    /// playing the sound.
    pub fn play_once(&self, sink: &Arc<Mutex<Sink>>) -> Result<(), Errors> {
        let source = rodio::buffer::SamplesBuffer::new(1, 44100, self.data.clone());
        let sink = sink.lock().unwrap();
        sink.append(source);
        Ok(())
    }

    /// Plays the sound in an infinite loop using the provided sink.
    ///
    /// # Arguments
    ///
    /// * `sink` - The sink to play the sound on.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `()` if successful, or an error of type `Errors` if there is an error
    /// playing the sound.

    pub fn play_loop(&self, sink: &Arc<Mutex<Sink>>) -> Result<(), Errors> {
        let source = rodio::buffer::SamplesBuffer::new(1, 44100, self.data.clone());
        let sink = sink.lock().unwrap();
        sink.append(source.repeat_infinite());
        Ok(())
    }
}
