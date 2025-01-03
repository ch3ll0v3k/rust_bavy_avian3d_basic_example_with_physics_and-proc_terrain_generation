// This shader computes the chromatic aberration effect

// Since post processing is a fullscreen effect, we use the fullscreen vertex shader provided by bevy.
// This will import a vertex shader that renders a single fullscreen triangle.
//
// A fullscreen triangle is a single triangle that covers the entire screen.
// The box in the top left in that diagram is the screen. The 4 x are the corner of the screen
//
// Y axis
//  1 |  x-----x......
//  0 |  |  s  |  . ´
// -1 |  x_____x´
// -2 |  :  .´
// -3 |  :´
//    +---------------  X axis
//      -1  0  1  2  3
//
// As you can see, the triangle ends up bigger than the screen.
//
// You don't need to worry about this too much since bevy will compute the correct UVs for you.
#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

struct PostProcessSettings {
  // intensity: f32,
  // set_r: f32,
  // set_g: f32,
  // set_b: f32,
  cam_y: f32,
  // #ifdef SIXTEEN_BYTE_ALIGNMENT
  //   // WebGL2 structs must be 16 byte aligned.
  //   _webgl2_padding: vec3<f32>
  // #endif
}
@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
@group(0) @binding(2) var<uniform> settings: PostProcessSettings;

@fragment
fn fragment(
  in: FullscreenVertexOutput
) -> @location(0) vec4<f32> {
  // Chromatic aberration strength
  // let set_r = settings.set_r;
  // let set_g = settings.set_g;
  // let set_b = settings.set_b;
  let cam_y = settings.cam_y;

  // min: -0.00096681714
  // max: -0.0020196673
  if( cam_y > -4.0 ){
    return vec4<f32>(
      textureSample(screen_texture, texture_sampler, in.uv).r,
      textureSample(screen_texture, texture_sampler, in.uv).g,
      textureSample(screen_texture, texture_sampler, in.uv).b,
      // textureSample(screen_texture, texture_sampler, in.uv + vec2<f32>(0.0, offset_strength)).b,
      1.0
    );
  }

  // let offset_strength = settings.intensity;

  return vec4<f32>(
    textureSample(screen_texture, texture_sampler, in.uv).r,
    textureSample(screen_texture, texture_sampler, in.uv + vec2<f32>(0.0, 0.005)).g,
    textureSample(screen_texture, texture_sampler, in.uv + vec2<f32>(0.0, 0.005)).b,
    1.0
  );
}

// textureSample(screen_texture, texture_sampler, in.uv + vec2<f32>(offset_strength, -offset_strength)).r,
// textureSample(screen_texture, texture_sampler, in.uv + vec2<f32>(-offset_strength, 0.0)).g,
// textureSample(screen_texture, texture_sampler, in.uv + vec2<f32>(0.0, offset_strength)).b,
