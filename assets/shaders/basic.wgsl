struct SceneUniform {
    color: vec4<f32>,
};

@group(0) @binding(0)
var<uniform> scene: SceneUniform;

@vertex
fn vs_main(@builtin(vertex_index) vid: u32) -> @builtin(position) vec4<f32> {
    var pos = array<vec2<f32>, 3>(
        vec2<f32>(0.0, 0.75),
        vec2<f32>(-0.75, -0.75),
        vec2<f32>(0.75, -0.75),
    );
    let p = pos[vid];
    return vec4<f32>(p, 0.0, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return scene.color;
}
