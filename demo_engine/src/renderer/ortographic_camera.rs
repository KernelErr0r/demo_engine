use crate::{Vec3, Mat4};

pub struct OrtographicCamera {
    projection_matrix: Mat4,
    view_matrix: Mat4,
    view_projection_matrix: Mat4,
    position: Vec3,
    rotation: f32,
}

impl OrtographicCamera {
    pub fn new(left: f32, right: f32, bottom: f32, top: f32) -> Self {
        let mut result = Self {
            projection_matrix: Mat4::orthographic_rh_gl(left, right, bottom, top, -1.0, 1.0),
            view_matrix: Mat4::identity(),
            view_projection_matrix: Mat4::zero(),
            position: Vec3::zero(),
            rotation: 0.0,
        };
        result.view_projection_matrix = result.projection_matrix * result.view_matrix;
        result
    }

    pub fn get_projection_matrix(&self) -> Mat4 {
        self.projection_matrix
    }

    pub fn get_view_matrix(&self) -> Mat4 {
        self.view_matrix
    }

    pub fn get_view_projection_matrix(&self) -> Mat4 {
        self.view_projection_matrix
    }

    pub fn get_position(&self) -> Vec3 {
        self.position
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
        self.recalculate_matrices();
    }

    pub fn get_rotation(&self) -> f32 {
        self.rotation
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
        self.recalculate_matrices();
    }

    fn recalculate_matrices(&mut self) {
        let transform = Mat4::from_translation(self.position)
            * Mat4::from_rotation_z(self.rotation.to_radians());

        self.view_matrix = transform.inverse();
        self.view_projection_matrix = self.projection_matrix * self.view_matrix;
    }
}