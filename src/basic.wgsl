

// structs 


[[block]] struct CameraUniform {
	view_matrix: mat4x4<f32>; 
	projection_matrix: mat4x4<f32>; 
};

[[block]] struct ModelUniform {
  model_matrix : mat4x4<f32>;
};


// bindings 
[[group(0), binding(0)]] var<uniform> camera: CameraUniform; 
[[group(0), binding(1)]] var<uniform> model : ModelUniform;


struct VertexInput {
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
struct InstanceInput {
    [[location(10)]] model_matrix_0: vec4<f32>;
    [[location(11)]] model_matrix_1: vec4<f32>;
    [[location(12)]] model_matrix_2: vec4<f32>;
    [[location(13)]] model_matrix_3: vec4<f32>;
};

struct VertexOutput {
    [[builtin(position)]] homogenous_clip_space_coords: vec4<f32>;
};

[[stage(vertex)]]
fn main(
  vertex: VertexInput,
	instance: InstanceInput,
) -> VertexOutput {
	
	let model_matrix = mat4x4<f32>(
		instance.model_matrix_0,
		instance.model_matrix_1,
		instance.model_matrix_2,
		instance.model_matrix_3,
	);

	var out: VertexOutput; 
	out.homogenous_clip_space_coords = camera.projection_matrix * camera.view_matrix * model.model_matrix * vertex.position;
	return out; 
}

// Fragment shader


[[stage(fragment)]]
fn main(in: VertexOutput) -> [[location(0)]] vec4<f32> {

    return vec4<f32>(0., 0., 1., 1.);
}