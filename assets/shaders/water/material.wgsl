// @group(0) @binding(0) var<storage, read_write> debug_output_arr: array<f32>;
// @group(0) @binding(0) var<storage, read_write> debug_output_f32: f32;

#define_import_path bevy_pbr::ssr

#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

#import bevy_pbr::{
  pbr_fragment::pbr_input_from_standard_material,
  pbr_functions::alpha_discard,
  prepass_utils,
}
#import bevy_pbr::{
  mesh_view_bindings::globals,
}

#ifdef PREPASS_PIPELINE
#import bevy_pbr::{
  prepass_io::{VertexOutput, FragmentOutput},
  pbr_deferred_functions::deferred_output,
}
#else
#import bevy_pbr::{
  forward_io::{VertexOutput, FragmentOutput},
  pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
}
#endif

struct MyExtendedMaterial {
  quantize_steps: u32,
}

@group(2) @binding(100)
var<uniform> my_extended_material: MyExtendedMaterial;

@fragment
fn fragment(
  #ifdef MULTISAMPLED
  @builtin(sample_index) sample_index: u32,
  #endif
  in: VertexOutput,
  @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
  // generate a PbrInput struct from the StandardMaterial bindings
  var pbr_input = pbr_input_from_standard_material(in, is_front);

  // we can optionally modify the input before lighting and alpha_discard is applied
  // pbr_input.material.base_color.b = pbr_input.material.base_color.r;
  // pbr_input.material.base_color.b = pbr_input.material.base_color.b * 0.5;

  // alpha discard
  pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);

  #ifdef PREPASS_PIPELINE
  // in deferred mode we can't modify anything after that, as lighting is run in a separate fullscreen shader.
  let out = deferred_output(in, pbr_input);
  #else
  var out: FragmentOutput;
  // apply lighting: def: on
  out.color = apply_pbr_lighting(pbr_input) * 0.5;

  // we can optionally modify the lit color before post-processing is applied
  // out.color = vec4<f32>(vec4<u32>(out.color * f32(my_extended_material.quantize_steps))) / f32(my_extended_material.quantize_steps);

  // apply in-shader post processing (fog, alpha-premultiply, and also tonemapping, debanding if the camera is non-hdr)
  // note this does not include fullscreen postprocessing effects like bloom.
  // out.color = main_pass_post_lighting_processing(pbr_input, out.color) * 1.0;

  // we can optionally modify the final result here
  // out.color = out.color * 0.5;
  let scene_depth = prepass_utils::prepass_depth(in.position, sample_index);
  let z = in.position.z* 1.0;
  // let depth_to_water_surface = in.position.z;
  // let water_depth = scene_depth - depth_to_water_surface;
  // let len = saturate(distance(depth_to_water_surface, water_depth));
  let water_depth = scene_depth / z;

  let f = 0.98;
  if water_depth > f {
    // out.color = mix(out.color, vec4(1.,1.,1.,0.5), smoothstep(f, 1., water_depth));
    out.color = mix(out.color, vec4(1.,1.,1.,0.5), smoothstep(f, 1., water_depth));
    // out.color = mix(out.color, vec4(0.5,0.5,0.5,0.1), smoothstep(f, 1., water_depth));
    out.color.a = 0.5;
  } else {
    out.color = out.color * smoothstep(0.2, 1., water_depth);
    // out.color.a = 0.01;
    // out.color = out.color * smoothstep(0.2, 1., water_depth);
  }

  // out.color = mix(out.color, vec4(1.,1.,1.,0.5), smoothstep(0.99, 1., water_depth));
  // out.color = out.color * smoothstep(0.2, 1., water_depth);

  // if water_depth > 0.99 {
  //   out.color = mix(out.color, vec4(1.,1.,1.,0.5), smoothstep(0.99, 1., water_depth));
  // } else {
  //   out.color = out.color * smoothstep(0.2, 1., water_depth);
  // }
  #endif

  return out;
}

#import bevy_pbr::{
  mesh_bindings::mesh,
  mesh_functions,
  skinning,
  morph::morph,
  forward_io::{Vertex},
  view_transformations::position_world_to_clip,
}

@vertex
fn vertex(vertex_no_morph: Vertex) -> VertexOutput {
  var out: VertexOutput;

#ifdef MORPH_TARGETS
  var vertex = morph_vertex(vertex_no_morph);
#else
  var vertex = vertex_no_morph;
#endif

#ifdef SKINNED
  var world_from_local = skinning::skin_model(vertex.joint_indices, vertex.joint_weights);
#else
  // Use vertex_no_morph.instance_index instead of vertex.instance_index to work around a wgpu dx12 bug.
  // See https://github.com/gfx-rs/naga/issues/2416 .
  var world_from_local = mesh_functions::get_world_from_local(vertex_no_morph.instance_index);
#endif

#ifdef VERTEX_NORMALS
#ifdef SKINNED
  out.world_normal = skinning::skin_normals(world_from_local, vertex.normal);
#else
  out.world_normal = mesh_functions::mesh_normal_local_to_world(
    vertex.normal,
    // Use vertex_no_morph.instance_index instead of vertex.instance_index to work around a wgpu dx12 bug.
    // See https://github.com/gfx-rs/naga/issues/2416
    vertex_no_morph.instance_index
  );
#endif
#endif

#ifdef VERTEX_POSITIONS
  var pos = vertex.position;
  pos.y = (
    sin(pos.x + globals.time) / 2.0
    +
    sin(pos.y + globals.time) / 3.0
    +
    sin(pos.x + globals.time) / 4.0
    +
    sin(pos.y + globals.time) / 6.0
  ) * 0.5;

  pos.x += (
    sin(pos.x + globals.time) / 2.0
    +
    sin(pos.x + globals.time) / 3.0
    +
    sin(pos.x + globals.time) / 4.0
    +
    sin(pos.x + globals.time) / 6.0
  ) * 2.5; // 5.0 like ocean

  pos.z += (
    sin(pos.z + globals.time) / 2.0
    +
    sin(pos.z + globals.time) / 3.0
    +
    sin(pos.z + globals.time) / 4.0
    +
    sin(pos.z + globals.time) / 6.0
  ) * 2.5; // 5.0 like ocean

  out.world_position = mesh_functions::mesh_position_local_to_world(world_from_local, vec4<f32>(pos, 1.0));
  out.position = position_world_to_clip(out.world_position.xyz);
#endif

#ifdef VERTEX_UVS_A
  out.uv = vertex.uv;
#endif
#ifdef VERTEX_UVS_B
  out.uv_b = vertex.uv_b;
#endif

#ifdef VERTEX_TANGENTS
  out.world_tangent = mesh_functions::mesh_tangent_local_to_world(
    world_from_local,
    vertex.tangent,
    // Use vertex_no_morph.instance_index instead of vertex.instance_index to work around a wgpu dx12 bug.
    // See https://github.com/gfx-rs/naga/issues/2416
    vertex_no_morph.instance_index
  );
#endif

#ifdef VERTEX_COLORS
  out.color = vertex.color;
#endif

#ifdef VERTEX_OUTPUT_INSTANCE_INDEX
  // Use vertex_no_morph.instance_index instead of vertex.instance_index to work around a wgpu dx12 bug.
  // See https://github.com/gfx-rs/naga/issues/2416
  out.instance_index = vertex_no_morph.instance_index;
#endif

#ifdef VISIBILITY_RANGE_DITHER
  out.visibility_range_dither = mesh_functions::get_visibility_range_dither_level(
    vertex_no_morph.instance_index, world_from_local[3]
  );
#endif
  return out;
}
