struct VertexOutput {
  @builtin(position) pos: vec4<f32>,
  @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(
  @location(0) position: vec2<f32>,
  @location(1) uv: vec2<f32>
) -> VertexOutput {
  var out: VertexOutput;

  out.pos = vec4<f32>(position, 0.0, 1.0);
  out.pos.y *= -1.0;
  out.uv = uv;

  return out;
}

@group(0) @binding(0) var box_texture: texture_2d<f32>;
@group(0) @binding(1) var box_sampler: sampler;

@fragment
fn fs_main(@location(0) uv_coords: vec2<f32>) -> @location(0) vec4<f32> {
  return textureSample(box_texture, box_sampler, uv_coords);
}
