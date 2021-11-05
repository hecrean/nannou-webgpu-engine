use super::GpuVertex;
use bytemuck::{Pod, Zeroable};
use nannou::wgpu;

#[repr(C)]
#[derive(Clone, Copy, Zeroable, Pod, Default)]
pub struct GltfMeshVertex {
    /*[[location(0)]] */ position: [f32; 4], //Vector4<f32>,
    /*[[location(1)]] */ normal: [f32; 3], //Vector3<f32>,
    /*[[location(2)]] */ tangent: [f32; 3], //Vector3<f32>,
    /*[[location(3)]] */ bitangent: [f32; 3], //Vector3<f32>,
    /*[[location(4)]] */ color: [f32; 4], //Vector4<f32>,
    /*[[location(5)]] */ tex_coords_0: [f32; 2], //Vector2<f32>,
    /*[[location(6)]] */ tex_coords_1: [f32; 2], //Vector2<f32>,
    /*[[location(7)]] */ tex_coords_2: [f32; 2], //Vector2<f32>,
    /*[[location(8)]] */ skin_weight: [f32; 3], //Vector3<f32>,
    /*[[location(9)]] */ skin_index: [f32; 3], //Vector3<f32>,
}

impl GpuVertex for GltfMeshVertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<GltfMeshVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                // position [12 bytes]
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // normal [12 bytes]
                wgpu::VertexAttribute {
                    offset: 12,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // tangent [12 bytes]
                wgpu::VertexAttribute {
                    offset: 24,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                },
                //bitangent [12 bytes]
                wgpu::VertexAttribute {
                    offset: 36,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // color [16 bytes]
                wgpu::VertexAttribute {
                    offset: 52,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x4,
                },
                // tex_coords_0 [8 bytes]
                wgpu::VertexAttribute {
                    offset: 60,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x2,
                },
                // tex_coords_1 [8 bytes]
                wgpu::VertexAttribute {
                    offset: 68,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x2,
                },
                // tex_coords_2 [8 bytes]
                wgpu::VertexAttribute {
                    offset: 76,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x2,
                },
                // skin_weight [12 bytes]
                wgpu::VertexAttribute {
                    offset: 88,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // skin_index [12 bytes]
                wgpu::VertexAttribute {
                    offset: 100,
                    shader_location: 9,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}
