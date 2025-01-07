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
    volume: f32,
    is_playing: bool,
    is_paused: bool,
    sink: Option<Arc<Mutex<Sink>>>, // Храним Arc<Mutex<Sink>>, а не MutexGuard
}

impl Sound {
    /// Creates a new sound from a file.
    pub fn new(file_path: &str) -> Result<Self, Errors> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let decoder = Decoder::new(reader)?;
        let data: Vec<u16> = decoder.convert_samples().collect();
        Ok(Self {
            data,
            volume: 1.0, // Default volume
            is_playing: false,
            is_paused: false,
            sink: None,
        })
    }

    /// Sets the volume of the sound (0.0 to 1.0).
    pub fn set_volume(&mut self, volume: f32) -> Result<(), Errors> {
        if volume < 0.0 || volume > 1.0 {
            return Err(Errors::AudioVolumeError(
                "Volume must be between 0.0 and 1.0".to_string(),
            ));
        }
        self.volume = volume;
        if let Some(sink) = &self.sink {
            let sink = sink.lock().unwrap();
            sink.set_volume(volume);
        }
        Ok(())
    }

    /// Checks if the sound is currently playing.
    pub fn is_playing(&mut self) -> bool {
        if let Some(sink) = &self.sink {
            let sink = sink.lock().unwrap();
            if sink.empty() {
                self.is_playing = false; // Обновляем флаг, если звук завершился
            }
        }
        self.is_playing
    }

    /// Checks if the sound is paused.
    pub fn is_paused(&self) -> bool {
        self.is_paused
    }

    /// Plays the sound once using the provided sink.
    pub fn play_once(&mut self, sink: &Arc<Mutex<Sink>>) -> Result<(), Errors> {
        let source = rodio::buffer::SamplesBuffer::new(1, 44100, self.data.clone());
        let sink = sink;
        sink.lock().unwrap().set_volume(self.volume);
        sink.lock().unwrap().append(source);
        self.is_playing = true;
        self.is_paused = false;
        self.sink = Some(Arc::clone(sink)); // Сохраняем Arc<Mutex<Sink>>, а не MutexGuard
        Ok(())
    }

    /// Plays the sound in a loop using the provided sink.
    pub fn play_loop(&mut self, sink: &Arc<Mutex<Sink>>) -> Result<(), Errors> {
        let source = rodio::buffer::SamplesBuffer::new(1, 44100, self.data.clone());
        let sink = sink;
        sink.lock().unwrap().set_volume(self.volume);
        sink.lock().unwrap().append(source.repeat_infinite());
        self.is_playing = true;
        self.is_paused = false;
        self.sink = Some(Arc::clone(sink)); // Сохраняем Arc<Mutex<Sink>>, а не MutexGuard
        Ok(())
    }

    /// Pauses the sound.
    pub fn pause(&mut self) -> Result<(), Errors> {
        if let Some(sink) = &self.sink {
            let sink = sink.lock().unwrap();
            sink.pause();
            self.is_playing = false;
            self.is_paused = true;
            Ok(())
        } else {
            self.is_playing = false;
            self.is_paused = true;
            Ok(())
        }
    }

    /// Resumes the sound.
    pub fn resume(&mut self) -> Result<(), Errors> {
        if let Some(sink) = &self.sink {
            let sink = sink.lock().unwrap();
            sink.play();
            self.is_playing = true;
            self.is_paused = false;
            Ok(())
        } else {
            self.is_playing = true;
            self.is_paused = false;
            Ok(())
        }
    }

    /// Stops the sound.
    pub fn stop(&mut self) -> Result<(), Errors> {
        if let Some(sink) = &self.sink {
            let sink = sink.lock().unwrap();
            sink.stop();
            self.is_playing = false;
            self.is_paused = false;
            Ok(())
        } else {
            self.is_playing = false;
            self.is_paused = false;
            Ok(())
        }
    }
}
