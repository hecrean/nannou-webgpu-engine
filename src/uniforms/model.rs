use bytemuck::{Pod, Zeroable};
use crevice::std140::AsStd140;
use mint::*;
use nannou::glam::{Mat4, Vec4};

#[derive(AsStd140, Clone, Copy)]
pub struct ModelUniform {
    model_matrix: ColumnMatrix4<f32>,
}
impl ModelUniform {
    pub fn new(model_matrix: Mat4) -> Self {
        Self {
            model_matrix: ColumnMatrix4::<f32>::from(model_matrix),
        }
    }
}
