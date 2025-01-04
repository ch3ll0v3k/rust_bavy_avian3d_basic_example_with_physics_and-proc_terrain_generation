use std::f32::consts::PI;

// prettier-ignore
use avian3d::prelude::{ 
  AngularVelocity, Collider, RigidBody, PhysicsSet, CollisionMargin,
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
const TERRAIN_CHUNK_Z: f32 = (1024.0 / TERRAIN_XZ_TO_Y_SCALLER) * 4.0; // 4.0
const TERRAIN_CHUNK_SUBDIVISIONS_SPLIT: u32 = 32; // 32
const SUBDIVISION_SUB_FACTOR: u32 = 1;
const TERRAIN_CHUNK_SCALLER: f64 = 1000.0; // 3à0.0
const TERRAIN_CHUNK_SUBDIVISIONS: u32 = 64; // 16 * 8

const MAX_TERRAIN_H_FOR_COLOR: f32 = 0.7336247; // 0.75;
const MIN_TERRAIN_H_FOR_COLOR: f32 = 0.28396824; // 0.25;
const TERRAIN_H_COLOR_STEP: f32 = (MAX_TERRAIN_H_FOR_COLOR - MIN_TERRAIN_H_FOR_COLOR) / 12.0;

static mut M_X: i32 = -10_000_000;
static mut M_Y: i32 = -10_000_000;

pub struct IInnerMap {
  pub entity: Entity,
  pub lod: i16,
}

#[derive(Resource, Default)]
pub struct InnerMapper {
  pub hash_map: HashMap<(i16, i16), IInnerMap>,
  // map: HashMap<(i16, i16), Entity> = HashMap::new(),
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
      .insert_resource(InnerMapper::new());

    // app
    //   .add_plugins((
    //     MaterialPlugin::<ExtendedMaterial<StandardMaterial, WaterExtension>>::default(),
    //     MaterialPlugin::<ExtendedMaterial<StandardMaterial, UnderWaterExtention>>::default(),
    //   ));

    app
      .add_systems(Startup, startup).add_systems(Update, update);
  }
}

// prettier-ignore
fn startup(
  mut res_mut_texture_cache: Option<ResMut</*res_mut_texture_cache::*/ImageCache>>,
  mut inner_mapper_mut: Option<ResMut<InnerMapper>>,
  asset_server: Res<AssetServer>,
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  // mut water_materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, WaterExtension>>>,
  // mut wat_materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, UnderWaterExtention>>>,
) {

  let image_hashmap: &mut ResMut<ImageCache> = res_mut_texture_cache.as_mut().unwrap();

  const UV_SCALE: f32 = 1.0; // 8.0; 
  let lod: [[i16; 13]; 13] = get_lod();
  let mut _min: f32 = f32::MAX;
  let mut _max: f32 = -f32::MAX;
  let segments:i32 = 3;

  let terrain_pbr_diff_handle: Handle<Image> = cache_load_image(
    image_hashmap,
    &asset_server, 
    pbr::aerial_grass_rock::AerialGrassRock::DiffLight.as_str(),
    true
  );

  let terrain_pbr_norm_handle: Handle<Image> = cache_load_image(
    image_hashmap,
    &asset_server, 
    pbr::aerial_grass_rock::AerialGrassRock::NorGl.as_str(),
    true
  );

  let terrain_pbr_rough_handle: Handle<Image> = cache_load_image(
    image_hashmap,
    &asset_server, 
    pbr::aerial_grass_rock::AerialGrassRock::Rough.as_str(),
    true
  );

  let terrain_pbr_ao_handle: Handle<Image> = cache_load_image(
    image_hashmap,
    &asset_server, 
    pbr::aerial_grass_rock::AerialGrassRock::Ao.as_str(),
    false
  );

  let mut terrain_material: StandardMaterial = StandardMaterial {
    base_color_texture: Some(terrain_pbr_diff_handle.clone()),
    normal_map_texture: Some(terrain_pbr_norm_handle.clone()),
    metallic_roughness_texture: Some(terrain_pbr_rough_handle.clone()),
    occlusion_texture: Some(terrain_pbr_ao_handle.clone()),
    // emissive_texture,
    // uv_transform: Affine2::from_scale(Vec2::new(16.0, 16.0)),
    // uv_transform: Affine2::from_scale(Vec2::new(8.0, 8.0)),
    uv_transform: Affine2::from_scale(Vec2::new(4.0, 4.0)),
    // uv_transform: Affine2::from_scale(Vec2::new(2.0, 2.0)),
    // uv_transform: Affine2::from_scale(Vec2::new(1.0, 1.0)),
    // uv_transform: Affine2::from_scale(Vec2::new(0.25, 0.25)),
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

  // terrain_material.uv_transform = Affine2::from_scale(Vec2::new(1.0, 1.0));
  let terrain_material_handle: Handle<StandardMaterial> = materials.add(terrain_material);

  // let water_diff_texture: Handle<Image> = cache_load_image(
  //   image_hashmap,
  //   &asset_server, 
  //   EImageWaterBase::Walet1Base.as_str(),
  //   true
  // );

  // let water_normal_map_texture: Handle<Image> = cache_load_image(
  //   image_hashmap,
  //   &asset_server, 
  //   EImageWaterBase::Walet1Normal.as_str(),
  //   true
  // );

  // let water_material = StandardMaterial{
  //   // base_color_texture: Some(water_diff_texture.clone()),
  //   // normal_map_texture: Some(water_normal_map_texture.clone()),
  //   // uv_transform: Affine2::from_scale(Vec2::new(4.0, 4.0)),
  //   // base_color: BLUE_400.into(),
  //   metallic: 0.3,
  //   base_color: Color::srgba_u8(128, 197, 222, 30),
  //   perceptual_roughness: 0.8,
  //   alpha_mode: AlphaMode::Blend,
  //   opaque_render_method: OpaqueRendererMethod::Auto,
  //   ..default()
  // };
  // let water_material_handle: Handle<StandardMaterial> = materials.add(water_material);
  // let water_material_handle: Handle<StandardMaterial> = materials.add(UnderWaterExtention {
  //   fog_height: 5.0,
  //   fog_color: Vec4::new(0.0, 0.5, 1.0, 1.0), // Light blue fog
  // });

  // let mut water = Mesh::from(
  //   Plane3d::default()
  //     .mesh()
  //     // .size(TERRAIN_CHUNK_X-(TERRAIN_CHUNK_X/2.0), TERRAIN_CHUNK_Z-(TERRAIN_CHUNK_Z/2.0))
  //     .size(TERRAIN_CHUNK_X, TERRAIN_CHUNK_Z)
  //     .subdivisions(0)
  //   )
  //   .with_generated_tangents()
  //   .unwrap();
  // water.compute_normals();

  let mut water: Mesh = Mesh::from(
    Cuboid::new(TERRAIN_CHUNK_X, 0.1, TERRAIN_CHUNK_Z))
    .with_generated_tangents()
    .unwrap();
  water.compute_normals();

  let water_material: StandardMaterial = StandardMaterial {
    // base_color_texture: Some(base_color_texture.clone()),
    // normal_map_texture: Some(water_normal_map_texture.clone()),
    // uv_transform: Affine2::from_scale(Vec2::new(4.0*4.0, 4.0*4.0)),
    // uv_transform: Affine2::from_scale(Vec2::new(4.0, 4.0)),
    // occlusion_texture: Some(water_normal_map_texture.clone()),
    // clearcoat: 0.1,
    // clearcoat_perceptual_roughness: 0.1,
    // metallic: 0.3,
    unlit: !false,
    double_sided: true,
    cull_mode: Some(Face::Front),
    // cull_mode: Some(Face::Back),
    // base_color: Color::srgba_u8(128, 197, 222, 30),
    base_color: Color::srgba_u8(70, 70, 180, 17),
    // perceptual_roughness: 0.8,
    opaque_render_method: OpaqueRendererMethod::Auto,
    alpha_mode: AlphaMode::Blend,
    ..default()
  };

  let water_material_handle = materials.add(water_material);
  // let water_material_handle: Handle<ExtendedMaterial<StandardMaterial, UnderWaterExtention>> = wat_materials.add(ExtendedMaterial {
  //   base: water_material,
  //   extension: UnderWaterExtention { 
  //     fog_height: 10.0, 
  //     fog_color: Vec4::new(0.0, 0.5, 1.0, 0.25),
  //     base_color: Vec4::new(0.5, 0.5, 0.5, 0.25),
  //   },
  // });
  

  // commands.spawn(PbrBundle {
  //   mesh: meshes.add(Mesh::from(shape::Plane { size: 50.0 })),
  //   material: materials.add(UnderWaterExtention {
  //     fog_height: 5.0,
  //     fog_color: Vec4::new(0.0, 0.5, 1.0, 1.0), // Light blue fog
  //   }),
  //   ..default()
  // });


  // let water_material = StandardMaterial {
  //   base_color_texture: Some(base_color_texture.clone()),
  //   normal_map_texture: Some(water_normal_map_texture.clone()),
  //   // uv_transform: Affine2::from_scale(Vec2::new(4.0*4.0, 4.0*4.0)),
  //   uv_transform: Affine2::from_scale(Vec2::new(4.0, 4.0)),
  //   // occlusion_texture: Some(water_normal_map_texture.clone()),
  //   clearcoat: 0.1,
  //   clearcoat_perceptual_roughness: 0.1,
  //   metallic: 0.3,
  //   base_color: BLUE_400.into(),
  //   perceptual_roughness: 0.8,
  //   opaque_render_method: OpaqueRendererMethod::Auto,
  //   alpha_mode: AlphaMode::Blend,
  //   ..default()
  // };
  // let water_material_handle = water_materials.add(ExtendedMaterial {
  //   base: water_material,
  //   extension: WaterExtension { quantize_steps: 30 },
  // });
  

  for z in -segments..=segments {
    for x in -segments..=segments {

      let on_z = ((lod.len() as i32) - 7 + z) as usize;
      let on_x = ((lod.len() as i32) - 7 + x) as usize;
      let dyn_scale = lod[ on_z ][ on_x ];

      if dyn_scale <= 0 {
        continue;
      } 

      let (terrain, min, max) = generate_chunk(x as f64, z as f64, UV_SCALE, dyn_scale);

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

      if let Some(res_mut) = &mut inner_mapper_mut {
        // dbgln!("capacity: {:?}", res_mut.hash_map.capacity());
        if let Some(res) = res_mut.hash_map.get(&(z as i16, x as i16)) {
          // dbgln!("res_mut.hash_map.get(&({z}, {x})) => lod: {}", res.lod);
        }else{
          // dbgln!("res_mut.hash_map.insert(&({z}, {x})) => lod: {dyn_scale}");
          let capacity = res_mut.hash_map.insert(
            (z as i16, x as i16), 
            IInnerMap{ 
                // entity: terrain_id, 
                entity: Entity::from( terrain_id ), 
                // entity: Entity::from_raw(42s31231231), 
                lod: dyn_scale
              }
          );
        }
      }

      let walter_f = 0;

      if z >= -walter_f && z <= walter_f && x >= -walter_f && x <= walter_f {
        // let mut water = Mesh::from(Cuboid::new(TERRAIN_CHUNK_X, 0.1, TERRAIN_CHUNK_Z));

        // if let Some(VertexAttributeValues::Float32x2(ref mut uvs)) = water.attribute_mut( Mesh::ATTRIBUTE_UV_0 ) {
        //   for uv in uvs.iter_mut() {
        //     // uv[0] *= 64.0; // Scale U
        //     // uv[1] *= 64.0; // Scale V
        //     // uv[0] *= 32.0; // Scale U
        //     // uv[1] *= 32.0; // Scale V
        //     uv[0] *= 16.0; // Scale U
        //     uv[1] *= 16.0; // Scale V
        //     // uv[0] *= 8.0; // Scale U
        //     // uv[1] *= 8.0; // Scale V
        //   }
        // }


        // let mat=materials.add(Color::srgba_u8(128, 197, 222,17));

        commands.spawn((
          // RigidBody::Static,
          // Collider::trimesh_from_mesh(&water).unwrap(),
          // Sensor,
          // Transform::from_translation(
          //   Vec3::new(
          //   (x * TERRAIN_CHUNK_X as i32) as f32, 
          //   10.125, 
          //   (z * TERRAIN_CHUNK_Z as i32) as f32
          //   )
          // ),
          Transform::from_xyz(
            (x * TERRAIN_CHUNK_X as i32) as f32, 
             -3.0, // -13
            (z * TERRAIN_CHUNK_Z as i32) as f32
            // .looking_at(Vec3::ZERO, Vec3::ZERO)
          ),
          Mesh3d(meshes.add(water.clone())),
          // MeshMaterial3d(materials.add(Color::srgba_u8(255, 40, 40, 30))),
          // MeshMaterial3d(materials.add(Color::srgba_u8(255, 40, 40, 250))),
          // MeshMaterial3d(materials.add(Color::srgba_u8(128, 197, 222,17))),
          // MeshMaterial3d(materials.add(Color::srgba_u8(128, 197, 222,30))),
          // MeshMaterial3d(water_material_handle.clone()),
          MeshMaterial3d(water_material_handle.clone()),
          // AngularVelocity(Vec3::new(2.5, 3.5, 1.5)),
          // DebugRender::default().with_collider_color(Color::srgb(1.0, 0.0, 1.0)),
          AnyObject,
          Name::new("water_t"),
        ));
      }
      
    }
  }

  // terrain: (min: -74.509224 max: 70.95005)

  dbgln!("terrain: (min: {_min} max: {_max})");

}

// prettier-ignore
fn update(
  mut inner_mapper_mut: Option<ResMut<InnerMapper>>,
  // inner_mapper_read: Res<InnerMapper>,
  // inner_mapper: Res<InnerMapper>,
  // mut q_terrain: Query<&mut Transform, (With<MTerrainMarker>, Without<PlayerMarker>)>,
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
  let m_x = ( (pos.x + (TERRAIN_CHUNK_X / 2.0)) / TERRAIN_CHUNK_X) as i32;
  let m_z = ( (pos.z + (TERRAIN_CHUNK_Z / 2.0)) / TERRAIN_CHUNK_Z) as i32;

  // if let Some(res_mut) = &mut inner_mapper_mut {
  //   dbgln!("----------------------------------------------");
  //   let keys: Vec<(i16, i16)> = res_mut.hash_map.iter().map(|(k, v)| k.clone()).collect::<Vec<(i16, i16)>>();
  //   for k in keys {
  //     let (z, x) = k;
  //     dbgln!("z: {z} / x: {x}");
  //   }
  // }

  unsafe {
  // dbgln!("player @: => {TERRAIN_CHUNK_X} => +>  (x: {m_x} z: {m_z} => p x/z => {}/{}", pos.x, pos.z);      

    if m_x != M_X || m_z != M_Y {
      M_X = m_x;
      M_Y = m_z;
      // dbgln!("player @: (x: {m_x} y: {m_z})");      
      dbgln!("player @: => {TERRAIN_CHUNK_X} => +>  (x: {m_x} z: {m_z} => p x/z => {}/{}", pos.x, pos.z);      

      // if let Some(res_mut) = &mut inner_mapper {
      //   // let mut res_mut = inner_mapper.unwrap
      //   let r = res_mut.state.get(&(0, 0)); // .unwrap();
      //   // res_mut.lod= 12;
      //   if let Some(in_map) = r {
      //       let item = in_map.get(&(0,0));
      //   }
      // }

      // inner_mapper.state.insert((0, 0), IInnerMap{ entity: Entity::from_bits(12), lod: 123});
      // let some = inner_mapper.state.get(&(0, 0)).unwrap();
      
      // let some = inner_mapper.state.get(&(0, 0));
      //   match Some(some)  {
      //   None => {
      //     dbgln!("inner_mapper.state.get(&(0, 0)): None");
      //     inner_mapper.state.insert((0, 0), IInnerMap{ entity: Entity::from_bits(12), lod: 123});
      //   }
      //   Some(x) => {
      //     let en = x.unwrap();
      //     dbgln!("inner_mapper.state.get(&(0, 0)): entity: {:?}, lod: {:?}", en.entity, en.lod);
      //   },
      // }

      // if let Some(x) = inner_mapper.state.get(&(0, 0)){
      //   dbgln!("inner_mapper.state.get(&(0, 0)): entity: {:?}, lod: {:?}", x.entity, x.lod);
      // }

      // let Option<&(Entity, i16)> = inner_mapper.state.get(&(0, 0));
      // let t: Option<&(Entity, i16)> = inner_mapper.state.get(&(0, 0));

    }
  }

}

fn load_base_texture(asset_server: &Res<AssetServer>, path: &str) -> Handle<Image> {
  let texture_handle: Handle<Image> = asset_server.load_with_settings(path, |s: &mut _| {
    *s = ImageLoaderSettings {
      sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
        // rewriting mode to repeat image,
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        // address_mode_w: ImageAddressMode::ClampToBorder,
        mag_filter: ImageFilterMode::Linear,
        ..default()
      }),
      ..default()
    };
  });

  texture_handle
}

fn calculate_final_subdivisions(dyn_scale: i16) -> u32 {
  let final_subdivisions: u32 =
    TERRAIN_CHUNK_SUBDIVISIONS / (dyn_scale as u32) - SUBDIVISION_SUB_FACTOR;
  return final_subdivisions;
}

// prettier-ignore
fn generate_chunk( x: f64, z: f64, uv_scale: f32, dyn_scale: i16 ) -> (Mesh, f32, f32) {
  
  let noise: BasicMulti<Perlin> = BasicMulti::<Perlin>::default();
  let final_subdivisions: u32 = calculate_final_subdivisions(dyn_scale);

  let mut terrain = Mesh::from(
    Plane3d::default()
      .mesh()
      .size(TERRAIN_CHUNK_X, TERRAIN_CHUNK_Z)
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
        (((pos[2] as f64) + (TERRAIN_CHUNK_Z as f64) * z) as f64) / TERRAIN_CHUNK_SCALLER,
        0.0 as f64,
      ]) as f32;
      pos[0] += (TERRAIN_CHUNK_X * (x as f32)) as f32; // + ((x / 1.0) as f32);
      pos[1] = xi * TERRAIN_HEIGHT * 1.0;
      // pos[1] = 0.0;
      pos[2] += (TERRAIN_CHUNK_Z * (z as f32)) as f32; // + ((z / 1.0) as f32);
      if use_segment_separator {
        pos[0] += (x / 1.0) as f32;
        pos[2] += (z / 1.0) as f32;
      }
    }

    // seconds pass
    // for pos in positions.iter_mut() {
    //   let xi: f32 = noise.get([
    //     (((pos[0] as f64) + (TERRAIN_CHUNK_X as f64) * x) as f64) / (TERRAIN_CHUNK_SCALLER / 100.0),
    //     (((pos[2] as f64) + (TERRAIN_CHUNK_Z as f64) * z) as f64) / (TERRAIN_CHUNK_SCALLER / 100.0),
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

    // let sub = 7.0; // -10.0;
    let sub = 15.0; // -10.0;

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

    if TERRAIN_USE_LOWER_Y_ON_FAR_DISTANCE{
      if let Some(VertexAttributeValues::Float32x3(positions)) = terrain.attribute_mut(Mesh::ATTRIBUTE_POSITION ){
        for pos in positions.iter_mut() {
          pos[1] -= (((dyn_scale.abs() as f32) - 4.0) * 1.0) / 2.0;
        }
      }
    }

  }

  // if let Some(VertexAttributeValues::Float32x2(ref mut uvs)) = terrain.attribute_mut( Mesh::ATTRIBUTE_UV_0 ) {
  //   for uv in uvs.iter_mut() {
  //     uv[0] *= uv_scale; // Scale U
  //     uv[1] *= uv_scale; // Scale V
  //   }
  // }

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
