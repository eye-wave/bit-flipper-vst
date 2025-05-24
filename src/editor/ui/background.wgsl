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
  out.uv = uv;

  return out;
}

@group(0) @binding(0) var bg_texture: texture_2d<f32>;
@group(0) @binding(1) var bg_sampler: sampler;

struct BackgroundUniforms {
  uv_region: vec4<f32>,
  time: f32,
};

@group(1) @binding(0) var<uniform> uniforms: BackgroundUniforms;

fn kaleidoscope_uv(uv: vec2<f32>, center: vec2<f32>, segments: f32) -> vec2<f32> {
  let centered = uv - center;

  let radius = length(centered);
  var angle = atan2(centered.y, centered.x) + uniforms.time * 0.05;

  let segment_angle = 2.0 * 3.14159265359 / segments;
  angle = abs(fract(angle / segment_angle) * 2.0 - 1.0) * segment_angle;
  
  return vec2<f32>(cos(angle), sin(angle)) * radius + center;
}

@fragment
fn fs_main(@location(0) uv_coords: vec2<f32>) -> @location(0) vec4<f32> {
    
  let normalized_uv = (uv_coords - uniforms.uv_region.xy) / uniforms.uv_region.zw;
  let folded_uv = kaleidoscope_uv(normalized_uv, vec2<f32>(0.5, 0.5), 6.0);

  let scaled_uv = (folded_uv + 0.5) * 2.0 + vec2(uniforms.time * 0.1,0.0);
  let wrapped_uv = fract(scaled_uv);
  let animated_uv = uniforms.uv_region.xy + wrapped_uv * uniforms.uv_region.zw;
    
  return textureSample(bg_texture, bg_sampler, animated_uv);
}
