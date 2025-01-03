@group(2) @binding(0) var<uniform> view_matrix: mat4x4<f32>;
@group(2) @binding(1) var<uniform> projection_matrix: mat4x4<f32>;

@fragment
fn fragment(
  @location(0) frag_coord: vec4<f32>,  // Fragment coordinates in screen space
  @texture_2d<float> color_texture: texture_2d<f32>,
  @sampler2d nearest_sampler: sampler2d
) -> @location(0) vec4<f32> {
  // Get the normalized device coordinates (NDC) from the frag coordinates
  let ndc = (frag_coord.xy / frag_coord.w) * 2.0 - 1.0;  // Normalize the coordinates to [-1, 1]

  // Calculate the height by using the Y component of NDC
  let height = ndc.y;

  // Fetch the color from the framebuffer
  let color = textureSample(color_texture, nearest_sampler, frag_coord.xy);

  // Apply a color effect based on the height
  let final_color = mix(color, vec4<f32>(1.0, 0.0, 0.0, 1.0), height * 0.5 + 0.5);

  return final_color;
}