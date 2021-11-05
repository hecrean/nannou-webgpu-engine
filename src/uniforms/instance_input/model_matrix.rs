use super::GpuInstance;
use bytemuck::{Pod, Zeroable};
use nannou::wgpu;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Default)]
pub struct ModelMatrixInstance {
    /*[[location(10)]] */ model_matrix_0: [f32; 4], //Vector4<f32>,
    /*[[location(11)]] */ model_matrix_1: [f32; 4], //Vector4<f32>,
    /*[[location(12)]] */ model_matrix_2: [f32; 4], //Vector4<f32>,
    /*[[location(13)]] */ model_matrix_3: [f32; 4], //Vector4<f32>,
}

impl GpuInstance for ModelMatrixInstance {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<ModelMatrixInstance>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Instance,
            attributes: &[
                // A mat4 takes up 4 vertex slots as it is technically 4 Vec4s.
                // model_matrix_0
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 10,
                    format: wgpu::VertexFormat::Float32x4,
                },
                // model_matrix_1
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 11,
                    format: wgpu::VertexFormat::Float32x4,
                },
                // model_matrix_2
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 12,
                    format: wgpu::VertexFormat::Float32x4,
                },
                // model_matrix_3
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 13,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}
