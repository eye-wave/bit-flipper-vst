struct VertexOutput {
  @builtin(position) pos: vec4<f32>
};

@vertex
fn vs_main(@location(0) position: vec2<f32>) -> VertexOutput {
  var out: VertexOutput;
  out.pos = vec4<f32>(position, 0.0, 1.0);
  return out;
}

@group(0) @binding(0)
var<uniform> box_color: vec4<f32>;

@fragment
fn fs_main() -> @location(0) vec4<f32> {
  return box_color;
}
