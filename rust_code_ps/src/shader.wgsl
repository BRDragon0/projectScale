struct Camera {
    view: mat4x4<f32>,
    proj: mat4x4<f32>,
    cpos: vec4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: Camera;

@group(1) @binding(1) var texture: texture_2d<f32>;
@group(1) @binding(2) var sample: sampler;

struct VertexOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) normals: vec3<f32>,
};

@vertex
fn vs_main(
    @location(0) pos: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) normals: vec3<f32>,
) -> VertexOut {
    var out: VertexOut;
    out.pos = camera.proj * camera.view * (vec4<f32>(pos, 1.0));
    out.uv = uv;
    out.normals = normals;
    return out;
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    return textureSample(texture, sample, in.uv);
}