use crevice::std140::AsStd140;
use mint::*;
use nannou::glam::{Mat4, Vec4};

#[derive(AsStd140, Clone, Copy)]
pub struct DirectionalLightUniforms {
    model_matrix: ColumnMatrix4<f32>,
    color: Vector4<f32>,
}

impl DirectionalLightUniforms {
    pub fn new(model_matrix: Mat4, color: Vec4) -> Self {
        Self {
            model_matrix: ColumnMatrix4::<f32>::from(model_matrix),
            color: Vector4::<f32>::from(color),
        }
    }
}
