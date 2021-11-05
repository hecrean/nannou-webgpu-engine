use nannou::prelude::Mat4;

use super::Handedness;
pub trait CameraProjection {
    fn projection_mat4(&self) -> Mat4;
    fn update(&mut self, width: usize, height: usize);
}

#[derive(Debug, Clone)]
pub struct PerspectiveProjection {
    pub handedness: Handedness,
    pub fov: f32,
    pub aspect_ratio: f32,
    pub near: f32,
    pub far: f32,
}

impl CameraProjection for PerspectiveProjection {
    // maps from from view/camera space to screen.space
    fn projection_mat4(&self) -> Mat4 {
        match self.handedness {
            Handedness::Right => {
                Mat4::perspective_rh(self.fov, self.aspect_ratio, self.near, self.far)
            }
            Handedness::Left => {
                Mat4::perspective_lh(self.fov, self.aspect_ratio, self.near, self.far)
            }
        }
    }

    fn update(&mut self, width: usize, height: usize) {
        self.aspect_ratio = width as f32 / height as f32;
    }
}

impl Default for PerspectiveProjection {
    fn default() -> Self {
        PerspectiveProjection {
            fov: std::f32::consts::PI / 4.0,
            near: 1.0,
            far: 1000.0,
            aspect_ratio: 1.0,
            handedness: Handedness::Right,
        }
    }
}

impl PerspectiveProjection {
    fn new(handedness: Handedness, fov: f32, aspect_ratio: f32, near: f32, far: f32) -> Self {
        Self {
            handedness,
            fov,
            aspect_ratio,
            near,
            far,
        }
    }
}
