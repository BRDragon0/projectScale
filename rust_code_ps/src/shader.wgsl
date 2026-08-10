struct Camera {
    view: mat4x4<f32>,
    proj: mat4x4<f32>,
    cpos: vec4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: Camera;

struct VertexOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(
    @location(0) pos: vec3<f32>,
    @location(1) color: vec3<f32>
) -> VertexOut {
    var out: VertexOut;
    //out.pos = camera.view * camera.proj * vec4<f32>(pos, 1.0);
    out.pos = camera.proj * camera.view * (vec4<f32>(pos, 1.0));
    out.color = color;
    return out;
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}