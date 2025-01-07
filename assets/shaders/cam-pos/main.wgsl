@group(2) @binding(100) var<uniform> height: f32;
@group(2) @binding(101) var<uniform> time_t: f32;

// @vertex
// fn vertex(@location(0) position: vec3<f32>) -> @builtin(position) vec4<f32> {
//   // var modified_position = position * vec3<f32>(camera_data.height, camera_data.height, 1.0);
//   // return vec4<f32>(modified_position, 1.0);
//   return vec4<f32>(position, 0.5);
// }

// @vertex
// fn vertex(@location(0) position: vec3<f32>) -> @builtin(position) vec4<f32> {
//   var modified_position = position * vec3<f32>(camera_data.height, camera_data.height, 1.0);
//   return vec4<f32>(position, 1.0);
// }

@fragment
fn fragment(
  @location(0) position: vec3<f32>,
  // @location(1) uv: vec2<f32> // UV coordinates from vertex shader,
) -> @location(0) vec4<f32> {

  let distortion = sin(time_t + position.x * 1.0) * 0.05;
  let distorted_y = position.y + distortion;  
  let base_y = 0.22;
  let y_add = 0.02;

  if( distorted_y > -2.25+y_add ){
    return vec4<f32>(0.0, 0.0, 0.0, 0.0);
  }

  if( distorted_y > -2.27+y_add ){
    return vec4<f32>(0.25, 0.25, 0.5,  0.05);
  }

  if( distorted_y > -2.29+y_add ){
    return vec4<f32>(0.35, 0.35, 0.5,  0.05);
  }

  // if( distorted_y > -2.27+y_add ){
  //   return vec4<f32>(0.35, 0.35, 0.5,  0.01);
  // }

  return vec4<f32>(0.0, 0.0, 0.5,  0.1);
}