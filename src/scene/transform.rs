//! Transform Module
//!
//! Represents a transformation in 3D space, including position, rotation, and scale.
//!
//! Transformations are used to position, rotate, and scale objects in a 3D scene.
//! The transformation matrix is updated lazily to optimize performance.
//!
//! # Example
//! ```rust
//! use glwfr::scene::transform::Transform;
//! use glwfr::cgmath::{Vector3, Deg, Quaternion};
//! let mut transform = Transform::new();
//! transform.set_position(Vector3::new(1.0, 2.0, 3.0));
//! transform.set_rotation(Quaternion::from_axis_angle(Vector3::unit_y(), Deg(45.0)));
//! transform.set_scale(Vector3::new(2.0, 2.0, 2.0));
//!
//! let matrix = transform.matrix(); // Get the transformation matrix
//! ```

use cgmath::*;

/// Represents a transformation in 3D space, including position, rotation, and scale.
#[derive(Debug, Clone)]
pub struct Transform {
    position: Vector3<f32>,
    rotation: Quaternion<f32>,
    scale: Vector3<f32>,
    dirty: bool,
    matrix: Matrix4<f32>,
}

impl Transform {
    /// Creates a new transformation with the default values:
    ///
    /// * position: `Vector3::new(0.0, 0.0, 0.0)`
    /// * rotation: `Quaternion::new(1.0, 0.0, 0.0, 0.0)`
    /// * scale: `Vector3::new(1.0, 1.0, 1.0)`
    ///
    /// # Returns
    ///
    /// A new `Transform` instance with the default values.
    pub fn new() -> Self {
        Self {
            position: Vector3::new(0.0, 0.0, 0.0),
            rotation: Quaternion::new(1.0, 0.0, 0.0, 0.0),
            scale: Vector3::new(1.0, 1.0, 1.0),
            dirty: true,
            matrix: Matrix4::identity(),
        }
    }

    /// Sets the position of the transform.
    ///
    /// The position is a vector describing the translation of the transform
    /// in 3D space.
    ///
    /// # Arguments
    ///
    /// * `position` - The new position of the transform.

    pub fn set_position(&mut self, position: Vector3<f32>) {
        self.position = position;
        self.dirty = true;
    }

    /// Translates the transform by the specified amount.
    ///
    /// This method adds the given translation vector to the current position of the transform.
    /// It is equivalent to calling `set_position` with the sum of the current position and the
    /// given translation vector.
    ///
    /// # Arguments
    ///
    /// * `translation` - The translation vector to add to the current position.
    pub fn translate(&mut self, translation: Vector3<f32>) {
        self.position += translation;
        self.dirty = true;
    }

    /// Sets the rotation of the transform.
    ///
    /// The rotation is a quaternion describing the orientation of the transform
    /// in 3D space.
    ///
    /// # Arguments
    ///
    /// * `rotation` - The new rotation of the transform as a quaternion.
    pub fn set_rotation(&mut self, rotation: Quaternion<f32>) {
        self.rotation = rotation;
        self.dirty = true;
    }

    /// Rotates the transform around the specified axis by the specified angle.
    ///
    /// This method modifies the rotation of the transform by applying a rotation
    /// around the given axis by the given angle. The rotation is applied in
    /// addition to the current rotation of the transform.
    ///
    /// # Arguments
    ///
    /// * `axis` - The axis of rotation.
    /// * `angle` - The angle of rotation in degrees.
    pub fn rotate_around_axis(&mut self, axis: Vector3<f32>, angle: Deg<f32>) {
        let rotation = Quaternion::from_axis_angle(axis.normalize(), Rad::from(angle));
        self.rotation = rotation * self.rotation;
        self.dirty = true;
    }

    /// Sets the scale of the transform.
    ///
    /// The scale is a vector describing the scaling of the transform
    /// in 3D space.
    ///
    /// # Arguments
    ///
    /// * `scale` - The new scale of the transform.
    pub fn set_scale(&mut self, scale: Vector3<f32>) {
        self.scale = scale;
        self.dirty = true;
    }

    /// Returns the position of the transform in 3D space.
    ///
    /// The position is a vector representing the translation of the transform
    /// along the x, y, and z axes.

    pub fn position(&self) -> Vector3<f32> {
        self.position
    }

    /// Returns the rotation of the transform.
    ///
    /// The rotation is a quaternion describing the orientation of the transform
    /// in 3D space.
    pub fn rotation(&self) -> Quaternion<f32> {
        self.rotation
    }

    /// Returns the scale of the transform.
    ///
    /// The scale is a vector describing the relative size of the transform
    /// on each axis.
    pub fn scale(&self) -> Vector3<f32> {
        self.scale
    }

    /// Returns the transformation matrix, updating it if necessary.
    ///
    /// If the transformation has been modified since the last time the matrix
    /// was calculated, this function will update the matrix by combining the
    /// translation, rotation, and scale components. Otherwise, it returns the
    /// previously calculated matrix.

    pub fn matrix(&mut self) -> Matrix4<f32> {
        if self.dirty {
            self.update_matrix();
        }
        self.matrix
    }

    /// Updates the transformation matrix by multiplying the translation, rotation, and scale matrices
    /// and sets the `dirty` flag to false. This function is called lazily when the transformation
    /// matrix is requested.
    fn update_matrix(&mut self) {
        let translation_matrix = Matrix4::from_translation(self.position);
        let rotation_matrix = Matrix4::from(self.rotation);
        let scale_matrix = Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z);

        self.matrix = translation_matrix * rotation_matrix * scale_matrix;
        self.dirty = false;
    }
}
