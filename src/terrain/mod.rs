use std::{ f32::consts::PI, time::Duration };

// prettier-ignore
use avian3d::prelude::{ 
  AngularVelocity, Collider, CollisionMargin, PhysicsSet, RigidBody, Sensor
};

// prettier-ignore
use bevy::{
  prelude::*,
  utils::default,
  gizmos::AppGizmoBuilder,
  asset::{ AssetServer, Assets, Handle },
  app::{ ScheduleRunnerPlugin, App, Startup, Update },
  audio::{ AudioPlugin, AudioPlayer, AudioSource, PlaybackSettings, PlaybackMode, Volume },
  image::{ ImageAddressMode, ImageFilterMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor },
  color::{ Color, palettes::css::*, palettes::tailwind::* },
  time::{ Time, Fixed, common_conditions::on_timer },
  math::{ IVec2, Vec2, Vec3, Affine2 },
  window::WindowMode::*,
  pbr::{ StandardMaterial, CascadeShadowConfigBuilder, ExtendedMaterial, OpaqueRendererMethod },
  ecs::query::{ QueryItem, QuerySingleError },
  core_pipeline::{
    core_3d::graph::{ Core3d, Node3d },
    fullscreen_vertex_shader::fullscreen_shader_vertex_state,
  },
  render::{
    mesh::VertexAttributeValues,
    extract_component::{ ComponentUniforms, DynamicUniformIndex, ExtractComponent, ExtractComponentPlugin, UniformComponentPlugin },
    render_graph::{ NodeRunError, RenderGraphApp, RenderGraphContext, RenderLabel, ViewNode, ViewNodeRunner },
    render_resource::{ binding_types::{ sampler, texture_2d, uniform_buffer } },
    renderer::{ RenderContext, RenderDevice },
    view::ViewTarget,
    RenderApp,
  },
};

use terrain_lod_map::TERRAIN_LOD_MAP_SIZE;
use wgpu::Face;
use noise::{ BasicMulti, NoiseFn, Perlin };
use std::collections::HashMap;

mod terrain_lod_map;

// prettier-ignore
use crate::{
  player::{ PlayerMarker },
  debug::get_defaul_physic_debug_params,
  AnyObject,
  PhysicsStaticObject,
  dbgln,
  PhysicsStaticObjectTerrain,
  COLLISION_MARGIN,
  sys_paths,
  terrain::terrain_lod_map::get_lod,
  asset_loader::image_cache::{ cache_load_image, ImageCache },
  // materials::water::{ UnderWaterExtention, WaterExtension },
};

// prettier-ignore
use sys_paths::{
  audio::EAudio,
  image::{EImageWaterBase,EImageTerrainBase, pbr},
};

#[derive(Component, Debug, PartialEq, Eq)]
pub struct MTerrainMarker;

#[derive(Component, Debug, PartialEq, Eq)]
pub struct MTerrainPlugin;

const TERRAIN_USE_LOWER_Y_ON_FAR_DISTANCE: bool = false;
const TERRAIN_XZ_TO_Y_SCALLER: f32 = 4.0; // 4.0;
const TERRAIN_HEIGHT: f32 = 70.0 * 2.0; // 70.0 * 2.0
const TERRAIN_CHUNK_X: f32 = (1024.0 / TERRAIN_XZ_TO_Y_SCALLER) * 4.0; // 4.0
const TERRAIN_CHUNK_X_HALF: f32 = TERRAIN_CHUNK_X / 2.0;
const TERRAIN_CHUNK_SUBDIVISIONS_SPLIT: u32 = 32; // 32
const SUBDIVISION_SUB_FACTOR: u32 = 1;
const TERRAIN_CHUNK_SCALLER: f64 = 1000.0; // 3à0.0
const TERRAIN_CHUNK_SUBDIVISIONS: u32 = 64; // 16 * 8

const MAX_TERRAIN_H_FOR_COLOR: f32 = 0.7336247; // 0.75;
const MIN_TERRAIN_H_FOR_COLOR: f32 = 0.28396824; // 0.25;
const TERRAIN_H_COLOR_STEP: f32 = (MAX_TERRAIN_H_FOR_COLOR - MIN_TERRAIN_H_FOR_COLOR) / 12.0;

const TERRAIN_STATIC_ON_MATERIAL_UV_SCALE: f32 = 4.0;
const TERRAIN_DYNAMIC_ON_MESH_UV_SCALE: f32 = 1.0;

const TERRAIN_SEGMENTS_TO_GEN: i32 = 2;

static mut M_X: i32 = -10_000_000;
static mut M_Y: i32 = -10_000_000;

pub struct IInnerMap {
  pub entity: Entity,
  pub lod: i16,
}

#[derive(Resource)]
pub struct IMapTestShift {
  pub x: f64,
  pub z: f64,
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
impl Plugin for MTerrainPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(InnerMapper::new())
      .insert_resource(IMapTestShift{ x: 0.0, z: 0.0 });

    app
      .add_systems(Startup, startup)
      // .add_systems(Update, (
      //   update_terrain_on_player_position,
      //   // modify_mesh_at_runtime,
      // ))
      .add_systems(Update, (
        update_terrain_on_player_position
      ).run_if(
        on_timer(Duration::from_millis(500))
      ));

  }
}

// prettier-ignore
fn get_terrain_bpr(
  asset_server: &Res<AssetServer>,
  image_hashmap: &mut ResMut<ImageCache>,
) -> StandardMaterial{

  let uv_transform: Vec2 = Vec2::new(
    TERRAIN_STATIC_ON_MATERIAL_UV_SCALE, 
    TERRAIN_STATIC_ON_MATERIAL_UV_SCALE
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

// prettier-ignore
fn startup(
  mut res_mut_texture_cache: Option<ResMut</*res_mut_texture_cache::*/ImageCache>>,
  mut inner_mapper_mut: Option<ResMut<InnerMapper>>,
  asset_server: Res<AssetServer>,
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {

  let image_hashmap: &mut ResMut<ImageCache> = res_mut_texture_cache.as_mut().unwrap();

 
  let mut water: Mesh = Mesh::from(
    Cuboid::new(TERRAIN_CHUNK_X, 0.1, TERRAIN_CHUNK_X))
    .with_generated_tangents()
    .unwrap();
    water.compute_normals();

  let water_material: StandardMaterial = StandardMaterial {
    unlit: !false,
    double_sided: true,
    cull_mode: Some(Face::Front),
    base_color: Color::srgba_u8(70, 70, 180, 17),
    opaque_render_method: OpaqueRendererMethod::Auto,
    alpha_mode: AlphaMode::Blend,
    ..default()
  };

  let water_material_handle = materials.add(water_material);

  let mut inner_map = inner_mapper_mut.as_mut().unwrap();

  // XXX

  let terrain_material: StandardMaterial = get_terrain_bpr(&asset_server, image_hashmap);
  let terrain_material_handle: Handle<StandardMaterial> = materials.add(terrain_material);

  let lod: [[i16; TERRAIN_LOD_MAP_SIZE]; TERRAIN_LOD_MAP_SIZE] = get_lod();
  let mut _min: f32 = f32::MAX;
  let mut _max: f32 = -f32::MAX;

  for z in -TERRAIN_SEGMENTS_TO_GEN..=TERRAIN_SEGMENTS_TO_GEN {
    for x in -TERRAIN_SEGMENTS_TO_GEN..=TERRAIN_SEGMENTS_TO_GEN {

      let on_z = ((lod.len() as i32) - 7 + z) as usize;
      let on_x = ((lod.len() as i32) - 7 + x) as usize;
      let dyn_scale = lod[ on_z ][ on_x ];

      if dyn_scale <= 0 {
        continue;
      } 

      let (terrain, min, max) = generate_chunk(x as f64, z as f64,  dyn_scale);

      _min = if _min > min { min } else { _min };
      _max = if _max < max { max } else { _max };

      let terrain_id = commands.spawn((
        RigidBody::Static,
        CollisionMargin(COLLISION_MARGIN),
        Collider::trimesh_from_mesh(&terrain).unwrap(),
        Mesh3d(meshes.add(terrain)),
        MeshMaterial3d(terrain_material_handle.clone()),
        // MeshMaterial3d(materials.add(Color::srgb_u8(255, 255, 255))),
        MTerrainMarker,
        PhysicsStaticObject,
        PhysicsStaticObjectTerrain,
        get_defaul_physic_debug_params(),
        AnyObject,
        Name::new("terrain_t"),
      )).id();


      if let Some(res) = inner_map.hash_map.get(&(z as i16, x as i16)) {
        dbgln!("inner_map.hash_map.get(&({z}, {x})) => lod (load): {}", res.lod);
      }else{
        dbgln!("inner_map.hash_map.insert(&({z}, {x})) => lod (dyn): {dyn_scale}");
        let capacity = inner_map.hash_map.insert(
          (z as i16, x as i16), 
          IInnerMap{ 
              // entity: terrain_id, 
              entity: Entity::from( terrain_id ), 
              lod: dyn_scale
            }
        );
      }

      // if let Some(res_mut) = &mut inner_mapper_mut {
      //   // dbgln!("capacity: {:?}", res_mut.hash_map.capacity());
      //   if let Some(res) = res_mut.hash_map.get(&(z as i16, x as i16)) {
      //     // dbgln!("res_mut.hash_map.get(&({z}, {x})) => lod: {}", res.lod);
      //   }else{
      //     // dbgln!("res_mut.hash_map.insert(&({z}, {x})) => lod: {dyn_scale}");
      //     let capacity = res_mut.hash_map.insert(
      //       (z as i16, x as i16), 
      //       IInnerMap{ 
      //           // entity: terrain_id, 
      //           entity: Entity::from( terrain_id ), 
      //           // entity: Entity::from_raw(42s31231231), 
      //           lod: dyn_scale
      //         }
      //     );
      //   }
      // }

      let walter_f = 0;

      if z >= -walter_f && z <= walter_f && x >= -walter_f && x <= walter_f {

        commands.spawn((
          // RigidBody::Static,
          // Collider::trimesh_from_mesh(&water).unwrap(),
          // Sensor,
          Transform::from_xyz(
            (x * TERRAIN_CHUNK_X as i32) as f32, 
             -3.0, // -13
            (z * TERRAIN_CHUNK_X as i32) as f32
            // .looking_at(Vec3::ZERO, Vec3::ZERO)
          ),
          Mesh3d(meshes.add(water.clone())),
          // MeshMaterial3d(materials.add(Color::srgba_u8(128, 197, 222,17))),
          // MeshMaterial3d(water_material_handle.clone()),
          MeshMaterial3d(water_material_handle.clone()),
          // DebugRender::default().with_collider_color(Color::srgb(1.0, 0.0, 1.0)),
          AnyObject,
          Name::new("water_t"),
        ));
      }
      
    }
  }

  dbgln!("terrain: (min: {_min} max: {_max})");

}

fn round_upto(num: f64, upto: i8) -> f64 {
  let pow = (10.0 as f64).powi(upto as i32);
  (num * pow).round() / pow
}

// prettier-ignore
fn update_terrain_on_player_position(
  mut inner_mapper_mut: Option<ResMut<InnerMapper>>,
  q_name: Query<&Name>,
  mut commands: Commands,
  mut q_player: Query<&mut Transform, (With<PlayerMarker>, Without<MTerrainMarker>)>,
  mut q_terrain: Query<
    (Entity, &mut RigidBody, &mut Transform),
    (With<MTerrainMarker>, Without<PlayerMarker>)
  >,
) {

  // return;


  let o_player = q_player.single_mut();
  let pos = o_player.translation;

  let add_x = if pos.x < 0.0 { -TERRAIN_CHUNK_X_HALF } else { TERRAIN_CHUNK_X_HALF };
  let add_z = if pos.z < 0.0 { -TERRAIN_CHUNK_X_HALF } else { TERRAIN_CHUNK_X_HALF };
  let x = ( (pos.x + add_x) / TERRAIN_CHUNK_X) as i32;
  let z = ( (pos.z + add_z) / TERRAIN_CHUNK_X) as i32;
  let lod: [[i16; TERRAIN_LOD_MAP_SIZE]; TERRAIN_LOD_MAP_SIZE] = get_lod();

  let on_z = 1;
  let on_x = 1;

  let dyn_scale = lod[ on_z ][ on_x ];

  if dyn_scale <= 0 {
    // dbgln!("dyn_scale: (HC) {dyn_scale}");
  } 

  let p_z = round_upto(pos.z as f64, 3);
  let p_x = round_upto(pos.x as f64, 3);

  dbgln!(" CH ({TERRAIN_CHUNK_X}) => lod: (z: {} / x: {}) => pos: (z: {} / x: {})", z, x, p_z, p_x);
  
  if let Some(res_mut) = &mut inner_mapper_mut {
    dbgln!("capacity: {:?}", res_mut.hash_map.capacity());
    if let Some(res) = res_mut.hash_map.get(&(z as i16, x as i16)) {
      dbgln!("res_mut.hash_map.get(&({z}, {x})) => lod: {}", res.lod);
    }else{
      // dbgln!("res_mut.hash_map.insert(&({z}, {x})) => lod: {dyn_scale}");
      // let capacity = res_mut.hash_map.insert(
      //   (z as i16, x as i16), 
      //   IInnerMap{ 
      //       // entity: terrain_id, 
      //       entity: Entity::from( terrain_id ), 
      //       // entity: Entity::from_raw(42s31231231), 
      //       lod: dyn_scale
      //     }
      // );
    }
  }


  // if let Some(res_mut) = &mut inner_mapper_mut {
  //   dbgln!("----------------------------------------------");
  //   let keys: Vec<(i16, i16)> = res_mut.hash_map.iter().map(|(k, v)| k.clone()).collect::<Vec<(i16, i16)>>();
  //   for k in keys {
  //     let (z, x) = k;
  //     dbgln!("z: {z} / x: {x}");
  //   }
  // }

}

fn calculate_final_subdivisions(dyn_scale: i16) -> u32 {
  let final_subdivisions: u32 =
    TERRAIN_CHUNK_SUBDIVISIONS / (dyn_scale as u32) - SUBDIVISION_SUB_FACTOR;
  return final_subdivisions;
}

// prettier-ignore
fn generate_chunk( x: f64, z: f64, dyn_scale: i16 ) -> (Mesh, f32, f32) {
  
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

  // dbgln!("chunk-size: {TERRAIN_CHUNK_X} => (base-subdiv: {TERRAIN_CHUNK_SUBDIVISIONS}, dyn_scale: {dyn_scale}) => final-subdiv: {final_subdivisions}");

  let use_segment_separator = false;
  let mut min: f32 = f32::MAX;
  let mut max: f32 = -f32::MAX;

  if let Some(VertexAttributeValues::Float32x3(positions)) = terrain.attribute_mut(Mesh::ATTRIBUTE_POSITION) {
    // main terrain topology
    for pos in positions.iter_mut() {
      let xi: f32 = noise.get([
        (((pos[0] as f64) + (TERRAIN_CHUNK_X as f64) * x) as f64) / TERRAIN_CHUNK_SCALLER,
        (((pos[2] as f64) + (TERRAIN_CHUNK_X as f64) * z) as f64) / TERRAIN_CHUNK_SCALLER,
        0.0 as f64,
      ]) as f32;
      pos[0] += (TERRAIN_CHUNK_X * (x as f32)) as f32; // + ((x / 1.0) as f32);
      pos[1] = xi * TERRAIN_HEIGHT * 1.0;
      // pos[1] = 0.0;
      pos[2] += (TERRAIN_CHUNK_X * (z as f32)) as f32; // + ((z / 1.0) as f32);
      if use_segment_separator {
        pos[0] += (x / 1.0) as f32;
        pos[2] += (z / 1.0) as f32;
      }
    }

    // seconds pass
    // for pos in positions.iter_mut() {
    //   let xi: f32 = noise.get([
    //     (((pos[0] as f64) + (TERRAIN_CHUNK_X as f64) * x) as f64) / (TERRAIN_CHUNK_SCALLER / 100.0),
    //     (((pos[2] as f64) + (TERRAIN_CHUNK_X as f64) * z) as f64) / (TERRAIN_CHUNK_SCALLER / 100.0),
    //     0.0 as f64,
    //   ]) as f32;
    //   pos[1] += xi * TERRAIN_HEIGHT * 0.001;
    // }

    for pos in positions.iter_mut() {
      pos[1] *= 1.50;
    }

    // waler down
    // for pos in positions.iter_mut() {
    //   pos[1] -= 10000.0; // def: 1.0
    // }

    for pos in positions.iter_mut() {
      pos[1] += 20.0; // def: 1.0
    }

    let sub = 7.0; // 15.0; // -10.0;

    let colors: Vec<[f32; 4]> = positions
      .iter()
      .map(|[_, g, _]| {
        let g: f32 = ((*g-sub) + TERRAIN_HEIGHT) / (TERRAIN_HEIGHT * 2.0); //  * 2.0 + 2.0; // * 26.0;
        min = if min > g { g } else { min };
        max = if max < g { g } else { max };
        return terrain_cal_color_on_g(g);
      })
      .collect();
    terrain.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    terrain.compute_normals();

    if TERRAIN_USE_LOWER_Y_ON_FAR_DISTANCE {
      if let Some(VertexAttributeValues::Float32x3(positions)) = terrain.attribute_mut(Mesh::ATTRIBUTE_POSITION ){
        for pos in positions.iter_mut() {
          pos[1] -= (((dyn_scale.abs() as f32) - 4.0) * 1.0) / 2.0;
        }
      }
    }

  }

  if let Some(VertexAttributeValues::Float32x2(ref mut uvs)) = terrain.attribute_mut( Mesh::ATTRIBUTE_UV_0 ) {
    for uv in uvs.iter_mut() {
      uv[0] *= TERRAIN_DYNAMIC_ON_MESH_UV_SCALE; // Scale U
      uv[1] *= TERRAIN_DYNAMIC_ON_MESH_UV_SCALE; // Scale V
    }
  }

  return (terrain, min, max);

}

// prettier-ignore
fn terrain_cal_color_on_g(g: f32) -> [f32; 4] {

  let mut color: [f32; 4];

  if g > MAX_TERRAIN_H_FOR_COLOR - TERRAIN_H_COLOR_STEP * 6.0 { 
    color = Color::from(WHITE).to_linear().to_f32_array();
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

fn round_to_two_digits(num: f64) -> f64 {
  (num * 100000000.0).round() / 100000000.0
}

// prettier-ignore
fn modify_mesh_at_runtime(
  mut res_mut_map_shift: Option<ResMut<IMapTestShift>>,
  mut meshes: ResMut<Assets<Mesh>>, 
  // query: Query<&Handle<Mesh>, With<MTerrainMarker>>,
  // mut qu: Query<(Entity, &mut Handle<Mesh3d>), With<MTerrainMarker>>,
  query: Query<&Mesh3d, With<MTerrainMarker>>,
) {

  let map: &mut ResMut<'_, IMapTestShift> = res_mut_map_shift.as_mut().unwrap();

  // map.x = 1.0;
  // map.z = 1.0;
  
  map.x = round_to_two_digits(map.x - 0.00000001);
  // map.z = round_to_two_digits(map.z + 0.00000001);
  dbgln!("map shift: (x: {} z: {})", map.x, map.z);
  
  // dbgln!(" ----------------------------------------------- ");
  // meshes.iter_mut().for_each(|(handle, mesh)| {
  //   dbgln!("mesh: {handle} => {:?}", mesh);
  // });

  let noise: BasicMulti<Perlin> = BasicMulti::<Perlin>::default();
  let final_subdivisions: u32 = calculate_final_subdivisions(4);
  // dbgln!("chunk-size: {TERRAIN_CHUNK_X} => (base-subdiv: {TERRAIN_CHUNK_SUBDIVISIONS}, dyn_scale: {dyn_scale}) => final-subdiv: {final_subdivisions}");
  let use_segment_separator = false;

  let mut x = map.x;
  let mut z = map.z;
  dbgln!("map shift: (x: {} z: {})", x, z);
  // let x = 0.0;
  // let z = 0.0;

  let dyn_scale: i32 = -1;

  let mut min: f32 = f32::MAX;
  let mut max: f32 = -f32::MAX;

  for mesh_handle in query.iter() {
    if let Some(terrain) = meshes.get_mut(mesh_handle) {
      // Modify the mesh data here, e.g., updating vertices or normals

      if let Some(VertexAttributeValues::Float32x3(positions)) = terrain.attribute_mut(Mesh::ATTRIBUTE_POSITION) {
        // main terrain topology
        for pos in positions.iter_mut() {
          let xi: f32 = noise.get([
            (((pos[0] as f64) + (TERRAIN_CHUNK_X as f64) * x) as f64) / TERRAIN_CHUNK_SCALLER,
            (((pos[2] as f64) + (TERRAIN_CHUNK_X as f64) * z) as f64) / TERRAIN_CHUNK_SCALLER,
            0.0 as f64,
          ]) as f32;
          pos[0] += (TERRAIN_CHUNK_X * (x as f32)) as f32; // + ((x / 1.0) as f32);
          pos[1] = xi * TERRAIN_HEIGHT * 1.0;
          // pos[1] = 0.0;
          pos[2] += (TERRAIN_CHUNK_X * (z as f32)) as f32; // + ((z / 1.0) as f32);
          // if use_segment_separator {
          //   pos[0] += (x / 1.0) as f32;
          //   pos[2] += (z / 1.0) as f32;
          // }
        }

        for pos in positions.iter_mut() {
          pos[1] *= 1.50;
        }

        // waler down
        // for pos in positions.iter_mut() {
        //   pos[1] -= 10000.0; // def: 1.0
        // }

        for pos in positions.iter_mut() {
          pos[1] += 20.0; // def: 1.0
        }

        let sub = 7.0; // 15.0; // -10.0;

        let colors: Vec<[f32; 4]> = positions
          .iter()
          .map(|[_, g, _]| {
            let g: f32 = ((*g-sub) + TERRAIN_HEIGHT) / (TERRAIN_HEIGHT * 2.0); //  * 2.0 + 2.0; // * 26.0;
            min = if min > g { g } else { min };
            max = if max < g { g } else { max };
            return terrain_cal_color_on_g(g);
          })
          .collect();
        terrain.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
        terrain.compute_normals();

        if TERRAIN_USE_LOWER_Y_ON_FAR_DISTANCE {
          if let Some(VertexAttributeValues::Float32x3(positions)) = terrain.attribute_mut(Mesh::ATTRIBUTE_POSITION ){
            for pos in positions.iter_mut() {
              pos[1] -= (((dyn_scale.abs() as f32) - 4.0) * 1.0) / 2.0;
            }
          }
        }
        
      }
    }
  }
}
