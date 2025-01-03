@group(2) @binding(101) var<uniform> fog_height: f32;
@group(2) @binding(102) var<uniform> fog_color: vec4<f32>;
@group(2) @binding(103) var<uniform> base_color: vec4<f32>;

@fragment
fn fragment(
  @location(0) in_world_position: vec3<f32>,
  // @location(1) base_color: vec4<f32>
) -> @location(0) vec4<f32> {
  return vec4(clamp(in_world_position.y, -50.0, 100.0), 0.0, 0.0, 0.25);
  // if( in_world_position.y > 0.5 ) return vec4(1.0, 0.0, 0.0, 0.25); }
  // return vec4(0.0, 1.0, 0.0, 0.25);
  // let fog_intensity = clamp((fog_height - in_world_position.y) / fog_height, 0.0, 0.25) * 0.25;
  // return mix(base_color, fog_color, fog_intensity);
  // return vec4(1.0, fog_height, 0.0, 1.0);
}



// @group(0) @binding(0) var<uniform> camera_position: vec3<f32>;
// @group(0) @binding(1) var<uniform> fog_height: f32;
// @group(0) @binding(2) var<uniform> fog_color: vec4<f32>;

// @fragment
// fn fragment(
//     @location(0) in_world_position: vec3<f32>,
// ) -> @location(0) vec4<f32> {
//     let fog_intensity = clamp((fog_height - in_world_position.y) / fog_height, 0.0, 1.0);
//     let base_color = vec4<f32>(0.0, 0.0, 1.0, 1.0); // Replace with your base color logic
//     return mix(base_color, fog_color, fog_intensity);
// }
