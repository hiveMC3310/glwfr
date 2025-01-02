use thiserror::Error;

#[derive(Error, Debug)]
pub enum Errors {
    #[error("Failed to initialize GLFW: {0}")]
    GlfwInitializationError(String),

    #[error("Failed to create window: {0}")]
    WindowCreationError(String),

    #[error("Failed to load texture: {0}")]
    TextureLoadError(String),

    #[error("Failed to compile shader: {0}")]
    ShaderCompilationError(String),

    #[error("Failed to link shader program: {0}")]
    ShaderLinkError(String),

    #[error("Failed to load file: {0}")]
    FileLoadError(String),

    #[error("OpenGL error: {0}")]
    OpenGlError(String),
}
