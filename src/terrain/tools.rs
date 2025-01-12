use std::collections::HashMap;

use noise::{ BasicMulti, NoiseFn, Perlin };

// prettier-ignore
use bevy::{
  app::{ App, ScheduleRunnerPlugin, Startup, Update }, 
  asset::{ AssetServer, Assets, Handle }, 
  audio::{ AudioPlayer, AudioPlugin, AudioSource, PlaybackMode, PlaybackSettings, Volume }, 
  color::{ 
    palettes::{css::*, tailwind::*}, 
    Color 
  }, 
  core_pipeline::{
    core_3d::graph::{ Core3d, Node3d },
    fullscreen_vertex_shader::fullscreen_shader_vertex_state,
  }, ecs::query::{ QueryItem, QuerySingleError }, 
  gizmos::AppGizmoBuilder, image::{ ImageAddressMode, ImageFilterMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor }, 
  math::{ Affine2, IVec2, Vec2, Vec3 }, 
  pbr::{ wireframe::Wireframe, CascadeShadowConfigBuilder, ExtendedMaterial, OpaqueRendererMethod, StandardMaterial }, 
  prelude::*, render::{
    extract_component::{ 
      ComponentUniforms, DynamicUniformIndex, ExtractComponent, ExtractComponentPlugin, UniformComponentPlugin 
    }, 
    mesh::VertexAttributeValues, render_graph::{ 
      NodeRunError, RenderGraphApp, RenderGraphContext, RenderLabel, ViewNode, ViewNodeRunner 
    }, 
    render_resource::binding_types::{ 
      sampler, texture_2d, uniform_buffer 
    }, 
    renderer::{ 
      RenderContext, RenderDevice 
    }, 
    view::ViewTarget, 
    RenderApp
  }, 
  time::{ 
    common_conditions::on_timer, Fixed, Time 
  }, utils::{ 
    default, hashbrown::hash_map
  }, window::WindowMode::*
};

use wgpu::Face;

use crate::{
  asset_loader::image_cache::{ cache_load_image, ImageCache },
  m_lib::colors,
  materials::water::WaterExtension,
  sys_paths::image::pbr,
};

use super::{ terrain_constants::*, terrain_lod_map::BASE_LOD_SCALE };

pub struct IInnerMap {
  pub entity: Entity,
  pub lod: i16,
}

#[derive(Resource, Default)]
pub struct InnerMapper {
  pub hash_map: HashMap<(i16, i16), IInnerMap>,
}

impl InnerMapper {
  pub fn new() -> Self {
    Self {
      hash_map: HashMap::new(),
    }
  }
}

// prettier-ignore
pub fn get_terrain_bpr(
  asset_server: &Res<AssetServer>,
  image_hashmap: &mut ResMut<ImageCache>,
  dyn_scale: i16
) -> StandardMaterial{
  let local_uv_scale = ( if dyn_scale == BASE_LOD_SCALE { 8.0 } else {1.0} );

  let uv_transform: Vec2 = Vec2::new(
    TERRAIN_STATIC_ON_MATERIAL_UV_SCALE * local_uv_scale,  
    TERRAIN_STATIC_ON_MATERIAL_UV_SCALE * local_uv_scale,
  );

  let terrain_pbr_diff_handle: Handle<Image> = cache_load_image(
    image_hashmap,
    asset_server, 
    pbr::aerial_grass_rock::AerialGrassRock::DiffLight.as_str(),
    true
  );

  let terrain_pbr_norm_handle: Handle<Image> = cache_load_image(
    image_hashmap,
    asset_server, 
    pbr::aerial_grass_rock::AerialGrassRock::NorGl.as_str(),
    true
  );

  let terrain_pbr_rough_handle: Handle<Image> = cache_load_image(
    image_hashmap,
    asset_server, 
    pbr::aerial_grass_rock::AerialGrassRock::Rough.as_str(),
    true
  );

  let terrain_pbr_ao_handle: Handle<Image> = cache_load_image(
    image_hashmap,
    asset_server, 
    pbr::aerial_grass_rock::AerialGrassRock::Ao.as_str(),
    true
  );

  let mut terrain_material: StandardMaterial = StandardMaterial {
    base_color_texture: Some(terrain_pbr_diff_handle.clone()),
    normal_map_texture: Some(terrain_pbr_norm_handle.clone()),
    metallic_roughness_texture: Some(terrain_pbr_rough_handle.clone()),
    occlusion_texture: Some(terrain_pbr_ao_handle.clone()),
    // emissive_texture,
    uv_transform: Affine2::from_scale(uv_transform),
    // alpha_mode: AlphaMode::Blend,
    unlit: false,
    emissive: LinearRgba::BLACK,
    // emissive_exposure_weight: 1.0,
    perceptual_roughness: 0.85,
    // metallic: 0.0,
    reflectance: 0.05,
    // ior: 1.47,
    ..default()
  };

  // terrain_material.uv_transform = Affine2::from_scale(Vec2::new(
  //   TERRAIN_STATIC_ON_MATERIAL_UV_SCALE, 
  //   TERRAIN_STATIC_ON_MATERIAL_UV_SCALE
  // ));

  terrain_material


}

pub fn get_water_pbr_and_mesh() -> (StandardMaterial, Mesh) {
  // let mut water: Mesh = Mesh::from(Cuboid::new(TERRAIN_CHUNK_X, 0.1, TERRAIN_CHUNK_X))
  //   .with_generated_tangents()
  //   .unwrap();
  // water.compute_normals();
  // let mut water: Mesh = Mesh::from(
  //   Cuboid::new(TERRAIN_CHUNK_X / 10.0, 0.1, TERRAIN_CHUNK_X / 10.0)
  // );

  let mut water = Mesh::from(
    Plane3d::default()
      .mesh()
      .size(TERRAIN_CHUNK_X / 10.0, TERRAIN_CHUNK_X / 10.0)
      .subdivisions(1)
  );
  // .with_generated_tangents()
  // .unwrap();

  let water_material: StandardMaterial = StandardMaterial {
    unlit: !false,
    double_sided: false,
    cull_mode: Some(Face::Back),
    // base_color: Color::srgba_u8(70, 70, 120, 1),
    base_color: Color::srgb_u8(70, 70, 120),
    // opaque_render_method: OpaqueRendererMethod::Auto,
    // alpha_mode: AlphaMode::Blend,
    ..default()
  };

  return (water_material, water);
}

pub fn calculate_final_subdivisions(dyn_scale: i16) -> u32 {
  TERRAIN_CHUNK_SUBDIVISIONS / (dyn_scale as u32) - SUBDIVISION_SUB_FACTOR
}

// prettier-ignore
fn gen_tree_at(x: f32, z: f32) {}

// prettier-ignore
pub fn generate_chunk( x: f64, z: f64, dyn_scale: i16 ) -> (Mesh, Vec<[f32; 3]>) {
  
  let tree_vec: Vec<[f32; 3]> = vec![];

  let noise: BasicMulti<Perlin> = BasicMulti::<Perlin>::default();
  let final_subdivisions: u32 = calculate_final_subdivisions(dyn_scale);

  let mut terrain = Mesh::from(
    Plane3d::default()
      .mesh()
      .size(TERRAIN_CHUNK_X, TERRAIN_CHUNK_X)
      .subdivisions(final_subdivisions)
  )
    .with_generated_tangents()
    .unwrap();


  if let Some(VertexAttributeValues::Float32x3(positions)) = terrain.attribute_mut(Mesh::ATTRIBUTE_POSITION) {

    // main terrain topology
    for pos in positions.iter_mut() {
      let g_x = pos[0];
      let g_z = pos[2];

      let xi: f32 = noise.get([
        (((g_x as f64) + (TERRAIN_CHUNK_X as f64) * x) as f64) / TERRAIN_CHUNK_SCALLER / 2.0,
        (((g_z as f64) + (TERRAIN_CHUNK_X as f64) * z) as f64) / TERRAIN_CHUNK_SCALLER / 2.0,
        0.0 as f64,
      ]) as f32;
      pos[0] += (TERRAIN_CHUNK_X * (x as f32)) as f32; // + ((x / 1.0) as f32);
      pos[1] = 0.0;
      pos[2] += (TERRAIN_CHUNK_X * (z as f32)) as f32; // + ((z / 1.0) as f32);

      // continue;

      let mut base_pos_y =  xi * TERRAIN_HEIGHT;

      // continue;

      // {
      //   // base topology: v1. base hills
      //   // pos[1] = base_pos_y;
      // }

      // {
      //   // dense map test
      //   let xi: f32 = noise.get([
      //     (((g_x as f64) + (TERRAIN_CHUNK_X as f64) * x) as f64) / TERRAIN_CHUNK_SCALLER / 10.0, // 10.0 == very smooth baseterrain 
      //     (((g_z as f64) + (TERRAIN_CHUNK_X as f64) * z) as f64) / TERRAIN_CHUNK_SCALLER / 10.0, // 10.0 == very smooth baseterrain 
      //     0.0 as f64,
      //   ]) as f32;
      //   let pos_y =  xi * TERRAIN_HEIGHT;
      //   pos[1] += pos_y * 3.0; // height
      // }

      // continue;

      {
        // base topology: v2. base hills
        let xi: f32 = noise.get([
          (((g_x as f64) + (TERRAIN_CHUNK_X as f64) * x) as f64) / TERRAIN_CHUNK_SCALLER / 10.0, // 10.0 == very smooth baseterrain 
          (((g_z as f64) + (TERRAIN_CHUNK_X as f64) * z) as f64) / TERRAIN_CHUNK_SCALLER / 10.0, // 10.0 == very smooth baseterrain 
          0.0 as f64,
        ]) as f32;
        let pos_y =  xi * TERRAIN_HEIGHT;
        pos[1] += pos_y * 3.0; // height

      }

      base_pos_y = pos[1];

      {
        // high hills pass
        let from = 300;
        let upto = 1000;
        let step = 25;
        for x in (-upto..=from).step_by(step){
          let y = (x as f64).abs() as f32;
          if y == 0.0 { continue; }
          if base_pos_y >= y as f32 {
            pos[1] += y;
            break;
          }
        }
      }

      // {
      //   // low-smoothing noise pass
      //   let xi: f32 = noise.get([
      //     (((g_x as f64) + (TERRAIN_CHUNK_X as f64) * x) as f64) / TERRAIN_CHUNK_SCALLER / 10.0,
      //     (((g_z as f64) + (TERRAIN_CHUNK_X as f64) * z) as f64) / TERRAIN_CHUNK_SCALLER / 10.0,
      //     0.0 as f64,
      //   ]) as f32;
      //   let pos_y =  xi * TERRAIN_HEIGHT;
      //   pos[1] += pos_y * 1.0;
      // }

      {
        // low-smoothing noise pass
        let xi: f32 = noise.get([
          (((g_x as f64) + (TERRAIN_CHUNK_X as f64) * x) as f64) / TERRAIN_CHUNK_SCALLER * 2.0,
          (((g_z as f64) + (TERRAIN_CHUNK_X as f64) * z) as f64) / TERRAIN_CHUNK_SCALLER * 2.0,
          0.0 as f64,
        ]) as f32;
        let pos_y =  xi * TERRAIN_HEIGHT;
        pos[1] += pos_y * 0.1;
      }


      {
        // base terrain height adjustment
        pos[1] *= 4.0;
        pos[1] += 20.0; // def: 1.0
      }

    }

    let sub = 7.0; // 15.0; // -10.0;

    let colors: Vec<[f32; 4]> = positions
      .iter()
      .map(|[_, g, _]| {
        let g: f32 = ((*g-sub) + TERRAIN_HEIGHT) / (TERRAIN_HEIGHT * 2.0); //  * 2.0 + 2.0; // * 26.0;

        // if( g > 0.5 ){
        //   return [0.0, 0.0, 1.0, 1.0];
        // }
        // return [1.0, 0.0, 0.0, 1.0];

        return terrain_cal_color_on_g(g);
      })
      .collect();
    terrain.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    terrain.compute_normals();

    if TERRAIN_USE_LOWER_Y_ON_FAR_DISTANCE && dyn_scale != BASE_LOD_SCALE {
      if let Some(VertexAttributeValues::Float32x3(positions)) = terrain.attribute_mut(Mesh::ATTRIBUTE_POSITION ){
        for pos in positions.iter_mut() {
          // pos[1] -= (((dyn_scale.abs() as f32) - 4.0) * 1.0) / 2.0;
          // pos[1] -= (((dyn_scale.abs() as f32) - 10.0) * 1.0) / 1.0;
          // pos[1] *= (((dyn_scale.abs() as f32 / 2.0))); // - 10.0) * 1.0) / 1.0;
          // pos[1] -= (((dyn_scale.abs() as f32 * 20.0))); // - 10.0) * 1.0) / 1.0;
          pos[1] -= (dyn_scale as f32 / 16.0 * 50.0 * (dyn_scale - BASE_LOD_SCALE) as f32); // (((dyn_scale.abs() as f32 * 2.0))); // - 10.0) * 1.0) / 1.0;
        }
      }
    }

  }

  // if( TERRAIN_DYNAMIC_ON_MESH_UV_SCALE > 1.0 ){
  //   if let Some(VertexAttributeValues::Float32x2(ref mut uvs)) = terrain.attribute_mut( Mesh::ATTRIBUTE_UV_0 ) {
  //     for uv in uvs.iter_mut() {
  //       uv[0] *= TERRAIN_DYNAMIC_ON_MESH_UV_SCALE; // Scale U
  //       uv[1] *= TERRAIN_DYNAMIC_ON_MESH_UV_SCALE; // Scale V
  //     }
  //   }
  // }

  return (terrain, tree_vec);

}

// prettier-ignore
fn terrain_cal_color_on_g(g: f32) -> [f32; 4] {

  let mut color: [f32; 4];


  if g > MAX_TERRAIN_H_FOR_COLOR - TERRAIN_H_COLOR_STEP * 6.0 { 
    // color = Color::from(WHITE).to_linear().to_f32_array();
    color = colors::hex_to_rgba_f32("#ffffff");
  } else if g > MAX_TERRAIN_H_FOR_COLOR - TERRAIN_H_COLOR_STEP * 6.3 {
    color = Color::from(GRAY_200).to_linear().to_f32_array();
  } else if g > MAX_TERRAIN_H_FOR_COLOR - TERRAIN_H_COLOR_STEP * 6.8 {
    color = Color::from(GRAY_300).to_linear().to_f32_array();
  } else if g > MAX_TERRAIN_H_FOR_COLOR - TERRAIN_H_COLOR_STEP * 7.3 {
    color = Color::from(GRAY_400).to_linear().to_f32_array();
  } else if g > MAX_TERRAIN_H_FOR_COLOR - TERRAIN_H_COLOR_STEP * 7.6 {
    color = Color::from(BLUE_500).to_linear().to_f32_array();
  } else if g > MAX_TERRAIN_H_FOR_COLOR - TERRAIN_H_COLOR_STEP * 7.9 {
    color = Color::from(BLUE_600).to_linear().to_f32_array();

  } else { // water-upper border
    color = Color::from(BLUE_600).to_linear().to_f32_array();
  }

  return color;

  // if g > MAX_TERRAIN_H_FOR_COLOR - TERRAIN_H_COLOR_STEP * 1.5 {
  //   color = Color::from(BLACK).to_linear().to_f32_array();
  // } else if g > MAX_TERRAIN_H_FOR_COLOR - TERRAIN_H_COLOR_STEP * 2.5 {
  //   color = Color::from(RED_500).to_linear().to_f32_array();
  // } else if g > MAX_TERRAIN_H_FOR_COLOR - TERRAIN_H_COLOR_STEP * 3.5 {
  //   color = Color::from(GREEN_500).to_linear().to_f32_array();
  // } else if g > MAX_TERRAIN_H_FOR_COLOR - TERRAIN_H_COLOR_STEP * 4.5 {
  //   color = Color::from(BLUE_500).to_linear().to_f32_array();
  // } else if g > MAX_TERRAIN_H_FOR_COLOR - TERRAIN_H_COLOR_STEP * 5.5 {
  //   color = Color::from(BLACK).to_linear().to_f32_array();
  // } else if g > MAX_TERRAIN_H_FOR_COLOR - TERRAIN_H_COLOR_STEP * 6.5 {
  //   color = Color::from(RED_500).to_linear().to_f32_array();
  // } else if g > MAX_TERRAIN_H_FOR_COLOR - TERRAIN_H_COLOR_STEP * 7.5 {
  //   color = Color::from(GREEN_500).to_linear().to_f32_array();
  // } else if g > MAX_TERRAIN_H_FOR_COLOR - TERRAIN_H_COLOR_STEP * 8.0 {
  //   color = Color::from(BLUE_500).to_linear().to_f32_array();
  // } else if g > MAX_TERRAIN_H_FOR_COLOR - TERRAIN_H_COLOR_STEP * 8.5 {
  //   color = Color::from(BLACK).to_linear().to_f32_array();
  // } else if g > MAX_TERRAIN_H_FOR_COLOR - TERRAIN_H_COLOR_STEP * 9.0 {
  //   color = Color::from(RED_500).to_linear().to_f32_array();
  // } else if g > MAX_TERRAIN_H_FOR_COLOR - TERRAIN_H_COLOR_STEP * 9.5 {
  //   color = Color::from(GREEN_500).to_linear().to_f32_array();
  // } else if g > MAX_TERRAIN_H_FOR_COLOR - TERRAIN_H_COLOR_STEP * 10.0 {
  //   color = Color::from(BLUE_500).to_linear().to_f32_array();
  // } else {
  //   color = Color::from(BLACK).to_linear().to_f32_array();
  // }
  // // color[3] = 0.1;
  // return color;

  if g > MAX_TERRAIN_H_FOR_COLOR - TERRAIN_H_COLOR_STEP * 2.0 {
    color = Color::from(GRAY_100).to_linear().to_f32_array();
  } else if g > MAX_TERRAIN_H_FOR_COLOR - TERRAIN_H_COLOR_STEP * 2.1 {
    color = Color::from(GRAY_200).to_linear().to_f32_array();
  } else if g > MAX_TERRAIN_H_FOR_COLOR - TERRAIN_H_COLOR_STEP * 2.2 {
    color = Color::from(GRAY_300).to_linear().to_f32_array();
  } else if g > MAX_TERRAIN_H_FOR_COLOR - TERRAIN_H_COLOR_STEP * 2.3 {
    color = Color::from(GRAY_300).to_linear().to_f32_array();
  } else if g > MAX_TERRAIN_H_FOR_COLOR - TERRAIN_H_COLOR_STEP * 2.4 {
    color = Color::from(GRAY_400).to_linear().to_f32_array();
  } else if g > MAX_TERRAIN_H_FOR_COLOR - TERRAIN_H_COLOR_STEP * 2.5 {
    color = Color::from(GRAY_400).to_linear().to_f32_array();
  } else if g > MAX_TERRAIN_H_FOR_COLOR - TERRAIN_H_COLOR_STEP * 3.0 {
    color = Color::from(GRAY_500).to_linear().to_f32_array();
  } else if g > MAX_TERRAIN_H_FOR_COLOR - TERRAIN_H_COLOR_STEP * 5.0 { // first TERRAIN_H_COLOR_STEP of mountens
    color = Color::from(GRAY_400).to_linear().to_f32_array();
  } else if g > MAX_TERRAIN_H_FOR_COLOR - TERRAIN_H_COLOR_STEP * 6.0 { // before mountens
    // color = Color::from(GREEN_500).to_linear().to_f32_array();
    color = Color::from(GRAY_500).to_linear().to_f32_array();
  } else if g > MAX_TERRAIN_H_FOR_COLOR - TERRAIN_H_COLOR_STEP * 6.2 {
    // color = Color::from(GREEN_200).to_linear().to_f32_array();
    color = Color::from(GRAY_200).to_linear().to_f32_array();
  } else if g > MAX_TERRAIN_H_FOR_COLOR - TERRAIN_H_COLOR_STEP * 6.5 {
    // color = Color::from(GREEN_100).to_linear().to_f32_array();
    color = Color::from(GRAY_100).to_linear().to_f32_array();
  } else if g > MAX_TERRAIN_H_FOR_COLOR - TERRAIN_H_COLOR_STEP * 7.3 { 
    // color = Color::from(GRAY_300).to_linear().to_f32_array();
    // color = Color::from(RED_500).to_linear().to_f32_array();
    // color = Color::from(GREEN_100).to_linear().to_f32_array();
    color = Color::from(GRAY_100).to_linear().to_f32_array();
  } else if g > MAX_TERRAIN_H_FOR_COLOR - TERRAIN_H_COLOR_STEP * 7.4 { // water-upper border
    color = Color::from(GRAY_300).to_linear().to_f32_array();
    // color = Color::from(RED_500).to_linear().to_f32_array();
  } else if g > MAX_TERRAIN_H_FOR_COLOR - TERRAIN_H_COLOR_STEP * 7.5 {// water-lower border
    color = Color::from(GRAY_300).to_linear().to_f32_array();
    // color = Color::from(RED_500).to_linear().to_f32_array();
  } else if g > MAX_TERRAIN_H_FOR_COLOR - TERRAIN_H_COLOR_STEP * 7.6 {
    color = Color::from(GRAY_400).to_linear().to_f32_array();
    // color = Color::from(RED_500).to_linear().to_f32_array();
  } else if g > MAX_TERRAIN_H_FOR_COLOR - TERRAIN_H_COLOR_STEP * 8.0 {
    // color = Color::from(BLUE_500).to_linear().to_f32_array();
    // color = Color::from(RED_500).to_linear().to_f32_array();
    // color = Color::from(BLUE_400).to_linear().to_f32_array();
    color = Color::from(GRAY_400).to_linear().to_f32_array();
  } else if g > MAX_TERRAIN_H_FOR_COLOR - TERRAIN_H_COLOR_STEP * 8.5 {
    // color = Color::from(BLUE_400).to_linear().to_f32_array();
    color = Color::from(GRAY_400).to_linear().to_f32_array();
  } else if g > MAX_TERRAIN_H_FOR_COLOR - TERRAIN_H_COLOR_STEP * 9.0 {
    // color = Color::from(BLUE_500).to_linear().to_f32_array();
    color = Color::from(GRAY_500).to_linear().to_f32_array();
  } else {
    // color = Color::from(BLUE_600).to_linear().to_f32_array();
    color = Color::from(GRAY_600).to_linear().to_f32_array();
  }

  return color;
}
