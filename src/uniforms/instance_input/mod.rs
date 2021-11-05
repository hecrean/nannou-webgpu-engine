pub mod model_matrix;
use nannou::wgpu;

pub trait GpuInstance {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a>;
}
