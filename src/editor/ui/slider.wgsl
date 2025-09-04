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
    let epsilon = 0.0001;

    let region_start = uniforms.uv_region.x;
    let region_width = max(uniforms.uv_region.z, epsilon);

    let normalized_uv = (uv_coords.x - region_start) / region_width;
    let scaled_uv = normalized_uv * (200.0 / 59.0) + (3800.0 / 3481.0);
    let clamped = clamp(scaled_uv - uniforms.value * (259.0 / 59.0), 0.0, 1.0);

    let denorm_x = region_start + clamped * region_width;
    let final_x = clamp(denorm_x, region_start + epsilon, region_start + region_width - epsilon);

    let denormalized_uv = vec2(final_x, uv_coords.y);
    return textureSample(box_texture, box_sampler, denormalized_uv);
}
