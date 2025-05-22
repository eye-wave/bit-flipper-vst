struct VertexOutput {
  @builtin(position) pos: vec4<f32>,
  @location(0) uv: vec2<f32>,
};

const VERTS = array<vec2<f32>, 6>(
  vec2<f32>(-1.0, -1.0),
  vec2<f32>( 1.0, -1.0),
  vec2<f32>( 1.0,  1.0),

  vec2<f32>( 1.0,  1.0),
  vec2<f32>(-1.0,  1.0),
  vec2<f32>(-1.0, -1.0),
);

@vertex
fn vs_main(@builtin(vertex_index) index: u32) -> VertexOutput {
  var out: VertexOutput;

  out.pos = vec4<f32>(VERTS[index], 0.0, 1.0);
  out.uv = VERTS[index] * 0.5 + 0.5;

  return out;
}

@group(0) @binding(0) var bg_texture: texture_2d<f32>;
@group(0) @binding(1) var bg_sampler: sampler;

@fragment
fn fs_main(@location(0) uv_coords: vec2<f32>) -> @location(0) vec4<f32> {
  let uv = vec2(uv_coords.x,1.0 - uv_coords.y);

  return textureSample(bg_texture, bg_sampler, uv);
}
