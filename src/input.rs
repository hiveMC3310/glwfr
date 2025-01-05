//! # Input Handling Module
//!
//! This module provides functionality for handling keyboard and mouse input in real-time.
//! It tracks the state of keys, mouse buttons, mouse position, and scroll events.
//!
//! ## Features
//! - Track pressed keys and mouse buttons.
//! - Get the current mouse position.
//! - Detect scroll events.
//! - Reset the input state.
//!
//! ## Usage
//! Just import the module and use the provided functions to handle input.
//!
//! ## Example
//! ```rust
//! use glwfr::input::{self, Key, MouseButton};
//!
//! fn handle_input(event: &WindowEvent) {
//!     input::process_event(event);
//!
//!     if input::is_key_pressed(Key::Right) {
//!         println!("Right arrow key is pressed!");
//!     }
//!
//!     if input::is_mouse_button_pressed(MouseButton::Left) {
//!         println!("Left mouse button is pressed!");
//!     }
//!
//!     let (x, y) = input::get_mouse_position();
//!     println!("Mouse position: ({}, {})", x, y);
//! }
//! ```

use glfw::{Action, WindowEvent};
pub use glfw::{Key, MouseButton};
use lazy_static::lazy_static;
use std::sync::Mutex;

// Static variables to store input state
lazy_static! {
    static ref KEYS_PRESSED: Mutex<[bool; 350]> = Mutex::new([false; 350]); // 350 - примерное количество клавиш
    static ref MOUSE_BUTTONS_PRESSED: Mutex<[bool; 8]> = Mutex::new([false; 8]); // 8 кнопок мыши
    static ref MOUSE_POSITION: Mutex<(f64, f64)> = Mutex::new((0.0, 0.0));
    static ref MOUSE_SCROLL: Mutex<(f64, f64)> = Mutex::new((0.0, 0.0));
}

/// Processes a `glfw::WindowEvent` to update the input state.
///
/// This function should be called for every event in your event loop.
///
/// # Example
/// ```rust
/// use glwfr::input;
/// use glfw::WindowEvent;
///
/// fn handle_event(event: &WindowEvent) {
///     input::process_event(event);
/// }
/// ```
pub fn process_event(event: &WindowEvent) {
    match event {
        WindowEvent::Key(key, _, Action::Press, _) => {
            KEYS_PRESSED.lock().unwrap()[*key as usize] = true;
        }
        WindowEvent::Key(key, _, Action::Release, _) => {
            KEYS_PRESSED.lock().unwrap()[*key as usize] = false;
        }
        WindowEvent::MouseButton(button, Action::Press, _) => {
            MOUSE_BUTTONS_PRESSED.lock().unwrap()[*button as usize] = true;
        }
        WindowEvent::MouseButton(button, Action::Release, _) => {
            MOUSE_BUTTONS_PRESSED.lock().unwrap()[*button as usize] = false;
        }
        WindowEvent::CursorPos(x, y) => {
            *MOUSE_POSITION.lock().unwrap() = (*x, *y);
        }
        WindowEvent::Scroll(xoffset, yoffset) => {
            *MOUSE_SCROLL.lock().unwrap() = (*xoffset, *yoffset);
        }
        _ => {}
    }
}

/// Checks if a specific key is currently pressed.
///
/// # Arguments
/// * `key` - The key to check (e.g., `Key::Right`).
///
/// # Returns
/// `true` if the key is pressed, `false` otherwise.
///
/// # Example
/// ```rust
/// use glwfr::input::{self, Key};
///
/// if input::is_key_pressed(Key::Space) {
///     println!("Space key is pressed!");
/// }
/// ```
pub fn is_key_pressed(key: Key) -> bool {
    KEYS_PRESSED.lock().unwrap()[key as usize]
}

/// Checks if a specific mouse button is currently pressed.
///
/// # Arguments
/// * `button` - The mouse button to check (e.g., `MouseButton::Left`).
///
/// # Returns
/// `true` if the button is pressed, `false` otherwise.
///
/// # Example
/// ```rust
/// use glwfr::input::{self, MouseButton};
///
/// if input::is_mouse_button_pressed(MouseButton::Left) {
///     println!("Left mouse button is pressed!");
/// }
/// ```
pub fn is_mouse_button_pressed(button: MouseButton) -> bool {
    MOUSE_BUTTONS_PRESSED.lock().unwrap()[button as usize]
}

/// Returns the current mouse position.
///
/// # Returns
/// A tuple `(x, y)` representing the mouse position.
///
/// # Example
/// ```rust
/// use glwfr::input;
///
/// let (x, y) = input::get_mouse_position();
/// println!("Mouse position: ({}, {})", x, y);
/// ```
pub fn get_mouse_position() -> (f64, f64) {
    *MOUSE_POSITION.lock().unwrap()
}

/// Returns the current mouse scroll offset.
///
/// # Returns
/// A tuple `(xoffset, yoffset)` representing the scroll offset.
///
/// # Example
/// ```rust
/// use glwfr::input;
///
/// let (x, y) = input::get_mouse_scroll();
/// println!("Scroll offset: ({}, {})", x, y);
/// ```
pub fn get_mouse_scroll() -> (f64, f64) {
    *MOUSE_SCROLL.lock().unwrap()
}

/// Resets the input state, clearing all pressed keys, mouse buttons, and resetting mouse position and scroll.
///
/// # Example
/// ```rust
/// use glwfr::input;
///
/// input::reset_state();
/// ```
pub fn reset_state() {
    KEYS_PRESSED.lock().unwrap().fill(false);
    MOUSE_BUTTONS_PRESSED.lock().unwrap().fill(false);
    *MOUSE_POSITION.lock().unwrap() = (0.0, 0.0);
    *MOUSE_SCROLL.lock().unwrap() = (0.0, 0.0);
}
