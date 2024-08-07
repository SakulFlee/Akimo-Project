struct VertexData {
    @builtin(vertex_index) vertex_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) tangent: vec3<f32>,
    @location(3) bitangent: vec3<f32>,
    @location(4) uv: vec2<f32>,
}

struct InstanceData {
    @location(5) model_space_matrix_0: vec4<f32>,
    @location(6) model_space_matrix_1: vec4<f32>,
    @location(7) model_space_matrix_2: vec4<f32>,
    @location(8) model_space_matrix_3: vec4<f32>,
}

struct FragmentData {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) tangent: vec3<f32>,
    @location(3) bitangent: vec3<f32>,
}

struct CameraUniform {
    view_projection_matrix: mat4x4<f32>,
}

@group(0) @binding(0) var albedo_texture: texture_2d<f32>;
@group(0) @binding(1) var albedo_sampler: sampler;

@group(1) @binding(0) 
var<uniform> camera: CameraUniform;

@vertex
fn entrypoint_vertex(
    vertex: VertexData,
    instance: InstanceData
) -> FragmentData {
    let model_space_matrix = mat4x4<f32>(
        instance.model_space_matrix_0,
        instance.model_space_matrix_1,
        instance.model_space_matrix_2,
        instance.model_space_matrix_3,
    );

    var out: FragmentData;

    // Calculate actual position
    out.position = camera.view_projection_matrix * model_space_matrix * vec4<f32>(vertex.position, 1.0);

    // Passthrough variables
    out.uv = vertex.uv;
    out.normal = vertex.normal;
    out.tangent = vertex.tangent;
    out.bitangent = vertex.bitangent;

    return out;
}

@fragment
fn entrypoint_fragment(fragment: FragmentData) -> @location(0) vec4<f32> {
    let tangent_basis = mat3x3<f32>(
        fragment.tangent,
        fragment.bitangent,
        fragment.normal
    );

    return textureSample(
        albedo_texture,
        albedo_sampler,
        fragment.uv
    );
}
