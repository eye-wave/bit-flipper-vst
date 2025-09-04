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

struct SliderUniforms {
  uv_region: vec4<f32>,
  value: f32,
};

@group(1) @binding(0) var<uniform> uniforms: SliderUniforms;

const VIEW_W: f32 = 200.0;
const THUMB_W: f32 = 19.0;
const SLIDER_W: f32 = 59.0;

@fragment
fn fs_main(@location(0) uv_coords: vec2<f32>) -> @location(0) vec4<f32> {
  let normalized_uv = (uv_coords.x - uniforms.uv_region.x) / uniforms.uv_region.z;
  let scaled_uv = normalized_uv * (VIEW_W / SLIDER_W) + (VIEW_W * THUMB_W) / (SLIDER_W * SLIDER_W);
  let clamped = clamp(scaled_uv - uniforms.value * (VIEW_W / SLIDER_W + 1.0),0.0,1.0);

  let denormalized_uv = vec2(uniforms.uv_region.x + clamped * uniforms.uv_region.z,uv_coords.y);
  
  return textureSample(box_texture, box_sampler, denormalized_uv);
}
