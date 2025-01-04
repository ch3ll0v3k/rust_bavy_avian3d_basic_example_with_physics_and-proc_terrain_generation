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

#import bevy_pbr::{
  mesh_view_bindings::globals,
}


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

  let cam_y = settings.cam_y;

  if( cam_y > 0.09 /*&& cam_y < 0.35*/ ){

    let dimensions: vec2<u32> = textureDimensions(screen_texture);
    let texel_size: vec2<f32> = 2.0 / vec2<f32>(dimensions);

    // Define a larger sampling kernel with weights
    let offsets = array<vec2<f32>, 20>(
      vec2(-2.0, -2.0), vec2(-2.0, -2.0),  vec2( 0.0, -2.0), vec2( 2.0, -2.0), vec2( 2.0, -2.0),
      vec2(-2.0,  0.0), vec2(-1.0,  0.0),  vec2( 0.0,  0.0), vec2( 1.0,  0.0), vec2( 2.0,  0.0),
      vec2(-2.0,  0.0), vec2(-1.0,  0.0),  vec2( 0.0,  0.0), vec2( 1.0,  0.0), vec2( 2.0,  0.0),
      vec2(-2.0, 21.0), vec2(-2.0,  2.0),  vec2( 0.0,  2.0), vec2( 2.0,  2.0), vec2( 2.0,  2.0)
    );

    let weights = array<f32, 20>(
      0.0625 * 2.0 ,0.0625 * 2.0, 0.125 * 2.0, 0.0625 * 2.0, 0.0625* 2.0,
      0.1250 * 2.0 ,0.1250,  0.25, 0.1250, 0.125* 2.0,
      0.0625 * 2.0 ,0.0625, 0.125, 0.0625, 0.0625* 2.0,
      0.0625 * 2.0 ,0.0625, 0.125 * 2.0, 0.0625 * 2.0, 0.0625* 2.0
    );

    var color: vec4<f32> = vec4(0.0);

    for (var i: u32 = 0; i < 20; i = i + 1) {
      let sample_uv = in.uv + offsets[i] * texel_size;
      color += textureSample(screen_texture, texture_sampler, sample_uv) * weights[i];
    }

    // color /= 20.0;

    return color * 0.5;
  }
  return vec4<f32>(
    textureSample(screen_texture, texture_sampler, in.uv).r * 2.0,
    textureSample(screen_texture, texture_sampler, in.uv).g * 2.0,
    textureSample(screen_texture, texture_sampler, in.uv).b * 2.0,
    1.0
  );

  // {
  //   // Distortion strength
  //   let strength: f32 = 0.01;
    
  //   // Time-based sine wave (optional for dynamic effect)
  //   let offset = vec2<f32>(
  //     sin(in.uv.y * 20.0) * strength,
  //     cos(in.uv.x * 20.0) * strength
  //   );

  //   let distorted_uv = in.uv + offset;

  //   // Sample the texture with distorted coordinates
  //   let color = textureSample(screen_texture, texture_sampler, distorted_uv);

  //   return color;
  // }

  // // Chromatic aberration strength
  // // let set_r = settings.set_r;
  // // let set_g = settings.set_g;
  // // let set_b = settings.set_b;
  // let cam_y = settings.cam_y;

  // // min: -0.00096681714
  // // max: -0.0020196673

  // // let nd = 2.0;
  // // let depth = 50.0;
  // // let f = (-depth + nd) / depth;

  // if( cam_y > 0.03 ){
  //   let f = cam_y / 10.0; // / 2.0;

  //   let vv = textureSample(screen_texture, texture_sampler, in.uv + sin( in.uv.x + in.uv.y ) );
  //   return vv;
  //   // return vec4<f32>(
  //   //   textureSample(screen_texture, texture_sampler, in.uv).r,
  //   //   textureSample(screen_texture, texture_sampler, in.uv + vec2<f32>(f, 0.0 )).g,
  //   //   textureSample(screen_texture, texture_sampler, in.uv + vec2<f32>(0.0, f )).b,
  //   //   1.0
  //   // );

  // }

  // // let offset_strength = settings.intensity;

  // return vec4<f32>(
  //   textureSample(screen_texture, texture_sampler, in.uv).r,
  //   textureSample(screen_texture, texture_sampler, in.uv).g,
  //   textureSample(screen_texture, texture_sampler, in.uv).b,
  //   1.0
  // );

  // // return vec4<f32>(
  // //   textureSample(screen_texture, texture_sampler, in.uv).r,
  // //   textureSample(screen_texture, texture_sampler, in.uv + vec2<f32>(f, f )).g,
  // //   textureSample(screen_texture, texture_sampler, in.uv + vec2<f32>(f, f )).b,
  // //   1.0
  // // );
}

// textureSample(screen_texture, texture_sampler, in.uv + vec2<f32>(offset_strength, -offset_strength)).r,
// textureSample(screen_texture, texture_sampler, in.uv + vec2<f32>(-offset_strength, 0.0)).g,
// textureSample(screen_texture, texture_sampler, in.uv + vec2<f32>(0.0, offset_strength)).b,
