pub mod projection;

use crate::transform::{Handedness, Transform, Transformable};
use crate::uniforms::camera::CameraUniform;
use nannou::prelude::{Mat4, Quat, Vec3};
use projection::CameraProjection;
pub trait Camera {
    fn projection(&self) -> &dyn CameraProjection;
    fn view_mat4(&self) -> Mat4;
}

pub struct BasicCamera<P: CameraProjection> {
    pub transform: Transform,
    pub projection: P,
}

impl<P: CameraProjection> From<BasicCamera<P>> for CameraUniform {
    fn from(basic_camera: BasicCamera<P>) -> Self {
        let view_matrix = basic_camera.view_mat4();
        let projection_matrix = basic_camera.projection().projection_mat4();
        CameraUniform::new(view_matrix, projection_matrix)
    }
}

impl<P: CameraProjection> Camera for BasicCamera<P> {
    fn projection(&self) -> &dyn CameraProjection {
        &self.projection
    }
    //  transform vertices from world-space to view/camera space
    fn view_mat4(&self) -> Mat4 {
        Mat4::inverse(&self.transform.mat4x4())
    }
}

impl<P> BasicCamera<P>
where
    P: CameraProjection,
{
    pub fn new(transform: Transform, camera_projection: P) -> Self {
        Self {
            transform,
            projection: camera_projection,
        }
    }

    pub fn focus_on_target(&mut self, target: Transform) -> Quat {
        let handedness = self.transform.handededness();

        match handedness {
            Handedness::Left => Quat::from_mat4(&Mat4::look_at_lh(
                self.transform.translation,
                target.translation,
                self.transform.y_axis(),
            )),
            Handedness::Right => Quat::from_mat4(&Mat4::look_at_rh(
                self.transform.translation,
                target.translation,
                self.transform.y_axis(),
            )),
        }
    }

    pub fn set_position(&mut self, position: Vec3) -> () {
        self.transform.translation = position;
    }

    pub fn set_rotation(&mut self, rotation: Quat) -> () {
        self.transform.rotation = rotation;
    }
}
