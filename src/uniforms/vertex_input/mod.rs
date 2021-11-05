pub mod gltf_mesh_vertex;
use nannou::wgpu;

pub trait GpuVertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a>;
}
