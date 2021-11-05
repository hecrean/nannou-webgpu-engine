mod camera;
mod transform;
mod uniforms;

use crate::transform::Transform;
use bytemuck::{Pod, Zeroable};
use camera::projection::PerspectiveProjection;
use camera::BasicCamera;
use crevice::std140::{AsStd140, Std140};
use nannou::prelude::*;
use nannou::wgpu::BufferInitDescriptor;
use std::borrow::Cow;
use std::marker::PhantomData;
use uniforms::camera::CameraUniform;
use uniforms::directional_light::DirectionalLightUniforms;
use uniforms::instance_input::model_matrix::ModelMatrixInstance;
use uniforms::instance_input::GpuInstance;
use uniforms::model::ModelUniform;
use uniforms::vertex_input::gltf_mesh_vertex::GltfMeshVertex;
use uniforms::vertex_input::GpuVertex;
pub struct BasicEntity {
    pub model_uniforms: &ModelUniform,
    pub vertices: &Vec<GltfMeshVertex>,
    pub indices: &Vec<i32>,
    pub instances: &Vec<ModelMatrixInstance>,
}
impl BasicEntity {
    pub fn new(
        model_uniforms: &ModelUniform,
        vertices: &Vec<GltfMeshVertex>,
        indices: &Vec<i32>,
        instances: &Vec<ModelMatrixInstance>,
    ) -> Self {
        Self {
            model_uniforms,
            vertices,
            indices,
            instances,
        }
    }
}

fn main() {
    nannou::app(model).update(update).run();
}
struct Model {
    draw_cxt: DrawContext,
}

pub struct DrawContext {
    // - global uniforms
    camera_uniforms: CameraUniform,
    directional_lights: Vec<DirectionalLightUniforms>,
    // - scene graph
    world: Vec<BasicEntity>,
    // - renderer
    // cmd_encoder: wgpu::CommandEncoder,
    // surface_texture: wgpu::Texture,
    // depth_texture: wgpu::Texture,
    pipelines: Vec<Box<dyn Drawable>>,
}

pub trait Drawable {
    fn draw(&self, render_pass: wgpu::RenderPass, entity: &BasicEntity) -> ();
}

struct BasicPipeline<Vertex, Instance, CameraUniform, ModelUniform>
where
    Vertex: GpuVertex,
    Instance: GpuInstance,
    CameraUniform: AsStd140,
    ModelUniform: AsStd140,
{
    shader_module: wgpu::ShaderModule,
    _vertex: PhantomData<Vertex>,
    vertices_buffer: wgpu::Buffer,
    _instance: PhantomData<Instance>,
    instances_buffer: wgpu::Buffer,
    indices_buffer: wgpu::Buffer,
    _camera_uniform: PhantomData<CameraUniform>,
    camera_uniform_buffer: wgpu::Buffer,
    _model_uniform: PhantomData<ModelUniform>,
    model_uniform_buffer: wgpu::Buffer,
    bind_group_0: wgpu::BindGroup,
    pipeline: wgpu::RenderPipeline,
}

impl<Vertex, Instance, CameraUniform, ModelUniform>
    BasicPipeline<Vertex, Instance, CameraUniform, ModelUniform>
where
    Vertex: GpuVertex + Pod + Zeroable,
    Instance: GpuInstance + Pod + Zeroable,
    CameraUniform: AsStd140 + Copy,
    ModelUniform: AsStd140 + Copy,
{
    fn new(
        device: &wgpu::Device,
        vertices: &Vec<Vertex>,
        indices: &Vec<i32>,
        instances: &Vec<Instance>,
        camera_uniform: &CameraUniform,
        model_uniform: &ModelUniform,
        sample_count: &u32,
        dst_format: &wgpu::TextureFormat,
        depth_format: &wgpu::TextureFormat,
    ) -> Self {
        // Load shader modules.
        let shader_str = include_str!("basic.wgsl");

        let shader_module = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(&shader_str)),
            flags: wgpu::ShaderFlags::default(),
            label: None,
        });

        // Create the vertex, normal and index buffers.

        let vertices_bytes = bytemuck::cast_slice(&vertices);
        let indices_bytes = bytemuck::cast_slice(&indices);
        let instances_bytes = bytemuck::cast_slice(&instances);
        let camera_uniform_bytes = camera_uniform.as_std140().as_bytes();
        let model_uniform_bytes = model_uniform.as_std140().as_bytes();

        let vertices_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: vertices_bytes,
            usage: wgpu::BufferUsage::VERTEX,
        });
        let indices_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: indices_bytes,
            usage: wgpu::BufferUsage::INDEX,
        });
        let instances_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: instances_bytes,
            usage: wgpu::BufferUsage::VERTEX,
        });
        let camera_uniform_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: camera_uniform_bytes,
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });
        let model_uniform_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: model_uniform_bytes,
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });

        let bind_group_0_layout = wgpu::BindGroupLayoutBuilder::new()
            .uniform_buffer(wgpu::ShaderStage::VERTEX_FRAGMENT, false)
            .uniform_buffer(wgpu::ShaderStage::VERTEX_FRAGMENT, false)
            .build(device);

        let bind_group_0 = wgpu::BindGroupBuilder::new()
            .buffer::<CameraUniform>(&camera_uniform_buffer, 0..1)
            .buffer::<ModelUniform>(&model_uniform_buffer, 0..1)
            .build(&device, &bind_group_0_layout);

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_0_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline =
            wgpu::RenderPipelineBuilder::from_layout(&pipeline_layout, &shader_module)
                .fragment_shader(&shader_module)
                .color_format(*dst_format)
                .color_blend(wgpu::BlendComponent::REPLACE)
                .alpha_blend(wgpu::BlendComponent::REPLACE)
                .add_vertex_buffer::<Vertex>(&wgpu::vertex_attr_array![
                    // [[location(0)]] position: vec4<f32>;
                    0 => Float32x4,
                    // [[location(1)]] normal: vec3<f32>;
                    1 => Float32x3,
                    // [[location(2)]] tangent: vec3<f32>;
                    2 => Float32x3,
                    // [[location(3)]] bitangent: vec3<f32>;
                    3 => Float32x3,
                    // [[location(4)]] color: vec4<f32>;
                    4 => Float32x4,
                    // [[location(5)]] tex_coords_0: vec2<f32>;
                    5 => Float32x2,
                    // [[location(6)]] tex_coords_1: vec2<f32>;
                    6 => Float32x2,
                    // [[location(7)]] tex_coords_2: vec2<f32>;
                    7 => Float32x2,
                    // [[location(8)]] skin_weight: vec3<f32>;
                    8 => Float32x3,
                    // [[location(9)]] skin_index: vec3<f32>;
                    9 => Float32x3,
                ])
                .add_instance_buffer::<Instance>(&[
                    // [[location(10)]] model_matrix_0: vec4<f32>;
                    wgpu::VertexAttribute {
                        shader_location: 10,
                        format: wgpu::VertexFormat::Float32x4,
                        offset: std::mem::size_of::<[f32; 4]>() as u64 * 0,
                    },
                    // [[location(11)]] model_matrix_1: vec4<f32>;
                    wgpu::VertexAttribute {
                        shader_location: 11,
                        format: wgpu::VertexFormat::Float32x4,
                        offset: std::mem::size_of::<[f32; 4]>() as u64 * 1,
                    },
                    // [[location(12)]] model_matrix_2: vec4<f32>;
                    wgpu::VertexAttribute {
                        shader_location: 12,
                        format: wgpu::VertexFormat::Float32x4,
                        offset: std::mem::size_of::<[f32; 4]>() as u64 * 2,
                    },
                    // [[location(13)]] model_matrix_3: vec4<f32>;
                    wgpu::VertexAttribute {
                        shader_location: 13,
                        format: wgpu::VertexFormat::Float32x4,
                        offset: std::mem::size_of::<[f32; 4]>() as u64 * 3,
                    },
                ])
                .depth_format(*depth_format)
                .sample_count(*sample_count)
                .build(device);

        BasicPipeline {
            shader_module,
            _vertex: PhantomData,
            vertices_buffer,
            _instance: PhantomData,
            instances_buffer,
            indices_buffer,
            _camera_uniform: PhantomData,
            camera_uniform_buffer,
            _model_uniform: PhantomData,
            model_uniform_buffer,
            bind_group_0,
            pipeline: render_pipeline,
        }
    }
}

impl<Vertex, Instance, CameraUniform, ModelUniform> Drawable
    for BasicPipeline<Vertex, Instance, CameraUniform, ModelUniform>
where
    Vertex: GpuVertex,
    Instance: GpuInstance,
    CameraUniform: AsStd140,
    ModelUniform: AsStd140,
{
    fn draw<'a>(&'a self, render_pass: wgpu::RenderPass<'a>, entity: &BasicEntity) -> () {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.bind_group_0, &[]);

        /*
        ```wgsl
        struct GltfMeshVertex {
          [[location(0)]] position: vec4<f32>;
          [[location(1)]] normal: vec3<f32>;
          [[location(2)]] tangent: vec3<f32>;
          [[location(3)]] bitangent: vec3<f32>;
          [[location(4)]] color: vec4<f32>;
          [[location(5)]] tex_coords_0: vec2<f32>;
          [[location(6)]] tex_coords_1: vec2<f32>;
          [[location(7)]] tex_coords_2: vec2<f32>;
          [[location(8)]] skin_weight: vec3<f32>;
          [[location(9)]] skin_index: vec3<f32>;
        };
        struct ModelMatrixInstance {
          [[location(10)]] model_matrix_0: vec4<f32>;
          [[location(11)]] model_matrix_1: vec4<f32>;
          [[location(12)]] model_matrix_2: vec4<f32>;
          [[location(13)]] model_matrix_3: vec4<f32>;
        };
        ```
                                                                                                                                    -- slot 1 --			-- slot 2 --
                render_pipeline = { vertex: wgpu::VertexState { buffers: &[Vertex::desc(), InstanceRaw::desc()], ..} , ..}
        */

        render_pass.set_vertex_buffer(0, self.vertices_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.instances_buffer.slice(..));

        render_pass.set_index_buffer(self.indices_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(
            0..entity.indices.len() as u32,
            0,
            0..entity.instances.len() as _,
        );
    }
}

fn model(app: &App) -> Model {
    let w_id = app.new_window().size(1024, 576).view(view).build().unwrap();

    // The gpu device associated with the window's swapchain
    let window = app.window(w_id).unwrap();
    let device = window.swap_chain_device();
    let dst_format = Frame::TEXTURE_FORMAT;
    let depth_format = wgpu::TextureFormat::Depth32Float;
    let msaa_samples = window.msaa_samples();
    let (win_w, win_h) = window.inner_size_pixels();

    let depth_texture = wgpu::TextureBuilder::new()
        .size([win_w, win_h])
        .format(depth_format)
        .usage(wgpu::TextureUsage::RENDER_ATTACHMENT)
        .sample_count(msaa_samples)
        .build(device);
    let depth_texture_view = depth_texture.view().build();

    // create world :

    // entitiy-1 :

    let model_uniform = ModelUniform::new(mat4(
        vec4(1., 0., 0., 0.),
        vec4(0., 1., 0., 0.),
        vec4(0., 0., 1., 0.),
        vec4(0., 0., 0., 1.),
    ));
    let vertices = vec![GltfMeshVertex::default()];
    let instances = vec![ModelMatrixInstance::default()];
    let indices = vec![0];

    let entity1 = BasicEntity::new(&model_uniform, &vertices, &indices, &instances);

    let world = vec![entity1];

    // camera
    let camera = BasicCamera::new(Transform::default(), PerspectiveProjection::default());
    let camera_uniforms = CameraUniform::from(camera);

    // lights
    let directional_lights = vec![DirectionalLightUniforms::new(
        mat4(
            vec4(1., 0., 0., 0.),
            vec4(0., 1., 0., 0.),
            vec4(0., 0., 1., 0.),
            vec4(0., 0., 0., 1.),
        ),
        vec4(1., 0., 0., 0.),
    )];

    let basic_pipeline = BasicPipeline::new(
        device,
        &vertices,
        &indices,
        &instances,
        &camera_uniforms,
        &model_uniform,
        &msaa_samples,
        &dst_format,
        &depth_format,
    );

    let pipelines: Vec<Box<dyn Drawable>> = vec![Box::new(basic_pipeline)];

    Model {
        draw_cxt: DrawContext {
            camera_uniforms,
            directional_lights,
            world,
            pipelines,
        },
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let mut encoder = frame.command_encoder();
    let mut render_pass = wgpu::RenderPassBuilder::new()
        .color_attachment(frame.swap_chain_texture(), |color| color)
        .begin(&mut encoder);

    for pipeline in model.draw_cxt.pipelines.iter() {
        for entity in model.draw_cxt.world.iter() {
            pipeline.draw(render_pass, entity);
        }
    }
}
