@group(2) @binding(0) var<uniform> camera_position : vec3;

@fragment
fn main(@location(0) frag_pos: vec2) -> @location(0) vec4 {
  // Color effect based on the camera's Y position
  let color = vec3(0.0, camera_position.y / 10.0, 0.0); // Modify as needed
  return vec4(color, 1.0);
}