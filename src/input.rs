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
//! To use this module, call `process_event` with `glfw::WindowEvent` to update the input state.
//! Then, use the provided functions to query the state of keys, mouse buttons, and other input data.
//!
//! ## Example
//! ```rust
//! use glwfr::input::{self, Key, MouseButton};
//! use glfw::WindowEvent;
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
use std::collections::HashSet;
use std::sync::{Mutex, OnceLock};

// Static variables to store input state
static KEYS_PRESSED: OnceLock<Mutex<HashSet<Key>>> = OnceLock::new();
static MOUSE_BUTTONS_PRESSED: OnceLock<Mutex<HashSet<MouseButton>>> = OnceLock::new();
static MOUSE_POSITION: OnceLock<Mutex<(f64, f64)>> = OnceLock::new();
static MOUSE_SCROLL: OnceLock<Mutex<(f64, f64)>> = OnceLock::new();

/// Initializes the input state if it hasn't been initialized yet.
fn ensure_init() {
    static INIT: OnceLock<()> = OnceLock::new();
    INIT.get_or_init(|| {
        KEYS_PRESSED.get_or_init(|| Mutex::new(HashSet::new()));
        MOUSE_BUTTONS_PRESSED.get_or_init(|| Mutex::new(HashSet::new()));
        MOUSE_POSITION.get_or_init(|| Mutex::new((0.0, 0.0)));
        MOUSE_SCROLL.get_or_init(|| Mutex::new((0.0, 0.0)));
    });
}

/// A helper function to safely access and modify a `Mutex`.
fn with_lock<T, F, R>(lock: &Mutex<T>, f: F) -> R
where
    F: FnOnce(&mut T) -> R,
{
    let mut guard = lock.lock().unwrap();
    f(&mut guard)
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
    ensure_init();

    match event {
        WindowEvent::Key(key, _, Action::Press, _) => {
            with_lock(KEYS_PRESSED.get().unwrap(), |keys| keys.insert(*key));
        }
        WindowEvent::Key(key, _, Action::Release, _) => {
            with_lock(KEYS_PRESSED.get().unwrap(), |keys| keys.remove(key));
        }
        WindowEvent::MouseButton(button, Action::Press, _) => {
            with_lock(MOUSE_BUTTONS_PRESSED.get().unwrap(), |buttons| {
                buttons.insert(*button)
            });
        }
        WindowEvent::MouseButton(button, Action::Release, _) => {
            with_lock(MOUSE_BUTTONS_PRESSED.get().unwrap(), |buttons| {
                buttons.remove(button)
            });
        }
        WindowEvent::CursorPos(x, y) => {
            with_lock(MOUSE_POSITION.get().unwrap(), |pos| *pos = (*x, *y));
        }
        WindowEvent::Scroll(xoffset, yoffset) => {
            with_lock(MOUSE_SCROLL.get().unwrap(), |scroll| {
                *scroll = (*xoffset, *yoffset)
            });
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
    ensure_init();
    with_lock(KEYS_PRESSED.get().unwrap(), |keys| keys.contains(&key))
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
    ensure_init();
    with_lock(MOUSE_BUTTONS_PRESSED.get().unwrap(), |buttons| {
        buttons.contains(&button)
    })
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
    ensure_init();
    with_lock(MOUSE_POSITION.get().unwrap(), |pos| *pos)
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
    ensure_init();
    with_lock(MOUSE_SCROLL.get().unwrap(), |scroll| *scroll)
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
    ensure_init();
    with_lock(KEYS_PRESSED.get().unwrap(), |keys| keys.clear());
    with_lock(MOUSE_BUTTONS_PRESSED.get().unwrap(), |buttons| {
        buttons.clear()
    });
    with_lock(MOUSE_POSITION.get().unwrap(), |pos| *pos = (0.0, 0.0));
    with_lock(MOUSE_SCROLL.get().unwrap(), |scroll| *scroll = (0.0, 0.0));
}
