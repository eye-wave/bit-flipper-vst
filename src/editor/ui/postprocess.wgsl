struct VertexOutput {
  @builtin(position) pos: vec4<f32>,
  @location(0) uv: vec2<f32>,
};

const VERTS = array(
  vec2<f32>(-1.0, -1.0),
  vec2<f32>(1.0, -1.0),
  vec2<f32>(-1.0, 1.0),

  vec2<f32>(-1.0, 1.0),
  vec2<f32>(1.0, -1.0),
  vec2<f32>(1.0, 1.0),
);

@vertex
fn vs_main(@builtin(vertex_index) idx: u32) -> VertexOutput {
  var out: VertexOutput;
  out.pos = vec4<f32>(VERTS[idx], 0.0, 1.0);
  out.uv  = (VERTS[idx] + 1.0) * 0.5;
  return out;
}


@group(0) @binding(0) var grayscale_tex: texture_2d<f32>;
@group(0) @binding(1) var palette_tex: texture_2d<f32>;
@group(0) @binding(2) var tex_sampler: sampler;

@fragment
fn fs_main(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
  let value = textureSample(grayscale_tex, tex_sampler, uv).r * 0.875 + 0.125;

  return textureSample(palette_tex, tex_sampler, vec2<f32>(value, 0.0));
}
