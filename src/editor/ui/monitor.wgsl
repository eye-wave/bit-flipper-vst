struct MonitorUniforms {
  position: vec4<f32>,
  color: f32,
};

@group(0) @binding(0)
var<uniform> uniforms: MonitorUniforms;

@vertex
fn vs_main(@location(0) y: f32, @builtin(vertex_index) idx: u32) -> @builtin(position) vec4<f32> {
  let x = f32(idx) / uniforms.position.z;
  
  let px = uniforms.position.x + (x / (200.0/uniforms.position.z) * 2.0);
  let py = uniforms.position.y + (y / (200.0/uniforms.position.w)) + (uniforms.position.w / 200.0);

  return vec4<f32>(px, py, 0.0, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
  return vec4(vec3(uniforms.color),1.0);
}
