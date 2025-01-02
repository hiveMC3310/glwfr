use glfw::{Action, Key, MouseButton, WindowEvent};
use std::collections::HashSet;
use std::sync::OnceLock;

pub mod input {
    use std::sync::Mutex;

    use super::*;

    static KEYS_PRESSED: OnceLock<Mutex<HashSet<Key>>> = OnceLock::new();
    static MOUSE_BUTTONS_PRESSED: OnceLock<Mutex<HashSet<MouseButton>>> = OnceLock::new();
    static MOUSE_POSITION: OnceLock<Mutex<(f64, f64)>> = OnceLock::new();
    static MOUSE_SCROLL: OnceLock<Mutex<(f64, f64)>> = OnceLock::new();

    /// Initialize static variables. This function is called by `is_key_pressed`, `is_mouse_button_pressed`, `get_mouse_position`, and `get_mouse_scroll` to ensure that the static variables are initialized when one of these functions is called.
    fn init() {
        KEYS_PRESSED.get_or_init(|| Mutex::new(HashSet::new()));
        MOUSE_BUTTONS_PRESSED.get_or_init(|| Mutex::new(HashSet::new()));
        MOUSE_POSITION.get_or_init(|| Mutex::new((0.0, 0.0)));
        MOUSE_SCROLL.get_or_init(|| Mutex::new((0.0, 0.0)));
    }

    /// Process a window event and store the state of the input devices in static variables.
    ///
    /// This function should be called every frame to keep the input state up to date.
    ///
    /// # Arguments
    ///
    /// * `event` - The window event to process.
    ///
    /// # Locks
    ///
    /// This function locks the `KEYS_PRESSED`, `MOUSE_BUTTONS_PRESSED`, `MOUSE_POSITION`, and `MOUSE_SCROLL` locks.
    pub fn process_event(event: &WindowEvent) {
        init();

        match event {
            WindowEvent::Key(key, _, Action::Press, _) => {
                KEYS_PRESSED.get().unwrap().lock().unwrap().insert(*key);
            }
            WindowEvent::Key(key, _, Action::Release, _) => {
                KEYS_PRESSED.get().unwrap().lock().unwrap().remove(key);
            }
            WindowEvent::MouseButton(button, Action::Press, _) => {
                MOUSE_BUTTONS_PRESSED
                    .get()
                    .unwrap()
                    .lock()
                    .unwrap()
                    .insert(*button);
            }
            WindowEvent::MouseButton(button, Action::Release, _) => {
                MOUSE_BUTTONS_PRESSED
                    .get()
                    .unwrap()
                    .lock()
                    .unwrap()
                    .remove(button);
            }
            WindowEvent::CursorPos(x, y) => {
                *MOUSE_POSITION.get().unwrap().lock().unwrap() = (*x, *y);
            }
            WindowEvent::Scroll(xoffset, yoffset) => {
                *MOUSE_SCROLL.get().unwrap().lock().unwrap() = (*xoffset, *yoffset);
            }
            _ => {}
        }
    }

    /// Check if a specific key is currently pressed.
    ///
    /// This function checks the state of the specified key and returns `true`
    /// if the key is pressed, otherwise `false`.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to check the pressed state for.
    ///
    /// # Returns
    ///
    /// `true` if the specified key is currently pressed, otherwise `false`.

    pub fn is_key_pressed(key: Key) -> bool {
        init();
        KEYS_PRESSED.get().unwrap().lock().unwrap().contains(&key)
    }

    /// Check if a specific mouse button is currently pressed.
    ///
    /// This function checks the state of the specified mouse button and returns
    /// `true` if the button is pressed, otherwise `false`.
    ///
    /// # Arguments
    ///
    /// * `button` - The mouse button to check the pressed state for.
    ///
    /// # Returns
    ///
    /// `true` if the specified mouse button is currently pressed, otherwise `false`.
    pub fn is_mouse_button_pressed(button: MouseButton) -> bool {
        init();
        MOUSE_BUTTONS_PRESSED
            .get()
            .unwrap()
            .lock()
            .unwrap()
            .contains(&button)
    }

    /// Get the current position of the mouse cursor in the window.
    ///
    /// # Returns
    ///
    /// A tuple containing the x and y coordinates of the mouse cursor in the window.
    pub fn get_mouse_position() -> (f64, f64) {
        init();
        *MOUSE_POSITION.get().unwrap().lock().unwrap()
    }

    /// Get the amount of mouse wheel scrolling that has occurred since the last call to this function.
    ///
    /// # Returns
    ///
    /// A tuple containing the horizontal and vertical scrolling amounts.
    pub fn get_mouse_scroll() -> (f64, f64) {
        init();
        *MOUSE_SCROLL.get().unwrap().lock().unwrap()
    }
}
