use crevice::std140::AsStd140;
use mint::*;
use nannou::glam::Mat4;

#[derive(AsStd140, Clone, Copy)]
pub struct CameraUniform {
    view_matrix: ColumnMatrix4<f32>,
    projection_matrix: ColumnMatrix4<f32>,
}

impl CameraUniform {
    pub fn new(view_matrix: Mat4, projection_matrix: Mat4) -> Self {
        Self {
            view_matrix: ColumnMatrix4::from(view_matrix),
            projection_matrix: ColumnMatrix4::from(projection_matrix),
        }
    }
}
