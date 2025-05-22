struct VertexInput {
  @location(0) pos: vec2<f32>,
  @location(1) uv: vec2<f32>,
};

struct VertexOutput {
  @builtin(position) pos: vec4<f32>,
  @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) index: u32, input: VertexInput) -> VertexOutput {
  var out: VertexOutput;

  out.pos = vec4<f32>(input.pos, 0.0, 1.0);
  out.uv = input.uv;

  return out;
}

@group(0) @binding(0) var bg_texture: texture_2d<f32>;
@group(0) @binding(1) var bg_sampler: sampler;
@group(0) @binding(2) var<uniform> is_active: u32;

@fragment
fn fs_main(@location(0) uv_coords: vec2<f32>) -> @location(0) vec4<f32> {
  let uv = vec2(uv_coords.x,1.0 - uv_coords.y);

  if is_active == 0 {
    discard;
  }

  return textureSample(bg_texture, bg_sampler, uv);
}
