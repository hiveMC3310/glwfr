use glfw::{Action, Context, Key, WindowEvent};
use std::sync::mpsc::Receiver;

pub struct Window {
    glfw: glfw::Glfw,
    window_handle: glfw::Window,
    events: Receiver<(f64, WindowEvent)>,
}

impl Window {
    /// Create a new window with the given dimensions and title.
    ///
    /// # Errors
    ///
    /// This function will return an error if GLFW fails to initialize or if the window cannot be created.
    ///
    /// # Arguments
    ///
    /// * `width` - The width of the window in pixels.
    /// * `height` - The height of the window in pixels.
    /// * `title` - The title of the window.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `Window` instance if successful, or an error string otherwise.
    pub fn new(width: u32, height: u32, title: &str) -> Result<Self, String> {
        let glfw = glfw::init(glfw::FAIL_ON_ERRORS).map_err(|e| e.to_string())?;

        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .ok_or("Failed to create GLFW window!")?;

        window.set_framebuffer_size_polling(true);
        window.set_key_polling(true);

        Ok(Self {
            glfw,
            window_handle: window,
            events,
        })
    }

    /// Initialize the OpenGL context for this window.
    ///
    /// # OpenGL Functions
    ///
    /// This function makes the window's OpenGL context current and loads the OpenGL function pointers.
    pub fn init_gl(&mut self) {
        self.window_handle.make_current();
        gl::load_with(|s| self.window_handle.get_proc_address(s) as *const _);
    }

    /// Check if the window should close.
    ///
    /// # Returns
    ///
    /// `true` if the window should close, otherwise `false`.
    pub fn should_close(&self) -> bool {
        self.window_handle.should_close()
    }

    /// Enable depth testing for this window.
    ///
    /// # OpenGL Functions
    ///
    /// This function is a wrapper around `glEnable(GL_DEPTH_TEST)`.
    pub fn enable_depth_test(&self) {
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
        }
    }

    /// Enable blending for this window.
    ///
    /// # OpenGL Functions
    ///
    /// This function is a wrapper around `glEnable(GL_BLEND)` and
    /// `glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA)`.
    pub fn enable_blend(&self) {
        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }
    }

    /// Clear the screen with the given color.
    ///
    /// # OpenGL Functions
    ///
    /// This function is a wrapper around `glClearColor` and `glClear`.
    ///
    /// # Arguments
    ///
    /// * `r` - The red component of the color.
    /// * `g` - The green component of the color.
    /// * `b` - The blue component of the color.
    /// * `a` - The alpha component of the color.
    pub fn clear(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            gl::ClearColor(r, g, b, a);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    /// Process window events and swap the front and back buffers.
    ///
    /// This must be called every frame to keep the window responsive.
    pub fn update(&mut self) {
        self.process_events();
        self.glfw.poll_events();
        self.window_handle.swap_buffers();
    }

    /// Process window events and update the window state accordingly.
    ///
    /// This function should be called every frame to keep the window responsive.
    fn process_events(&mut self) {
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    // Make sure the viewport matches the new window dimensions.
                    unsafe { gl::Viewport(0, 0, width, height) }
                }
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window_handle.set_should_close(true)
                }
                _ => {}
            }
        }
    }
}
