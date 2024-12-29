#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unreachable_code)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_braces)]
#![allow(unused_parens)]

// use avian3d::debug_render::DebugRender;
use avian3d::debug_render::PhysicsDebugPlugin;
use avian3d::prelude::*;
use avian3d::PhysicsPlugins;
// use bevy::audio::AudioPlugin;
use bevy::audio::AudioPlayer;
use bevy::color::palettes::css::BLACK;
use bevy::color::palettes::css::SILVER;
use bevy::color::palettes::css::WHITE_SMOKE;
use bevy::image::ImageAddressMode;
use bevy::image::ImageFilterMode;
use bevy::image::ImageLoaderSettings;
use bevy::image::ImageSampler;
use bevy::image::ImageSamplerDescriptor;
use camera::CameraParentMarker;
use noise::{ BasicMulti, NoiseFn, Perlin };
// use bevy_window::WindowLevel;

use bevy::ecs::query::QuerySingleError;
use bevy::prelude::*;
use bevy::render::mesh::VertexAttributeValues;
use bevy::window::PresentMode::*;
use bevy::window::WindowMode::*;
use bevy_window::WindowResolution;
use bevy::color::palettes::tailwind::*;
// use bevy::{ color::palettes::tailwind::* };

use bevy::math::Affine2;

use camera::CameraMarker;
use debug::get_defaul_physic_debug_params;
// use entities::with_children::MEntityBigSphere;
use lights::MPointLightMarker;

mod camera;
mod cubes;
mod debug;
mod lights;
mod markers;
mod constants;
mod terrain;
mod entities;

use markers::m_avian::*;
use markers::m_bevy::*;
use constants::viewport_settings::*;
use constants::physics_world::*;
use terrain::MTerrainMarker;

// const TERRAIN_XZ_TO_Y_SCALLER: f32 = 8.0;
// const TERRAIN_HEIGHT: f32 = 70.0;
// const TERRAIN_CHUNK_X: f32 = 1024.0 / TERRAIN_XZ_TO_Y_SCALLER;
// const TERRAIN_CHUNK_Z: f32 = 1024.0 / TERRAIN_XZ_TO_Y_SCALLER;
// const TERRAIN_CHUNK_SUBDIVISIONS_SPLIT: u32 = 32 / 4;
// const TERRAIN_CHUNK_SCALLER: f64 = 300.0;
// // prettier-ignore
// const TERRAIN_CHUNK_SUBDIVISIONS: u32 = (TERRAIN_CHUNK_SUBDIVISIONS_SPLIT / (TERRAIN_XZ_TO_Y_SCALLER as u32)) * 2;

const TERRAIN_XZ_TO_Y_SCALLER: f32 = 4.0; // 4.0;
const TERRAIN_HEIGHT: f32 = 70.0 * 2.0; // 70.0 * 2.0
const TERRAIN_CHUNK_X: f32 = (1024.0 / TERRAIN_XZ_TO_Y_SCALLER) * 4.0; // 4.0
const TERRAIN_CHUNK_Z: f32 = (1024.0 / TERRAIN_XZ_TO_Y_SCALLER) * 4.0; // 4.0
const TERRAIN_CHUNK_SUBDIVISIONS_SPLIT: u32 = 32; // 32
const TERRAIN_CHUNK_SCALLER: f64 = 1000.0; // 3à0.0
// prettier-ignore
// const TERRAIN_CHUNK_SUBDIVISIONS: u32 = (TERRAIN_CHUNK_SUBDIVISIONS_SPLIT / (TERRAIN_XZ_TO_Y_SCALLER as u32)) * 1;
// const TERRAIN_CHUNK_SUBDIVISIONS: u32 = 32; // 16 * 8; // 16 * 8
// const TERRAIN_CHUNK_SUBDIVISIONS: u32 = 16 * 8; // 16 * 8
const TERRAIN_CHUNK_SUBDIVISIONS: u32 = 128; // 16 * 8

const MAX_TERRAIN_H_FOR_COLOR: f32 = 0.7336247; // 0.75;
const MIN_TERRAIN_H_FOR_COLOR: f32 = 0.28396824; // 0.25;
const TERRAIN_H_COLOR_STEP: f32 = (MAX_TERRAIN_H_FOR_COLOR - MIN_TERRAIN_H_FOR_COLOR) / 12.0;

const WINDOW_POSITIONS_DEV_SIDE_33_PERCENT: Vec2 = Vec2::new(800.0, 1100.0);
const WINDOW_POSITIONS_DEV_SIDE_50_PERCENT: Vec2 = Vec2::new(950.0, 1100.0);
static USE_WIN_SIZE: Vec2 = WINDOW_POSITIONS_DEV_SIDE_50_PERCENT;

#[derive(Resource)]
struct SoundtrackPlayer {
  track_list: Vec<Handle<AudioSource>>,
}

impl SoundtrackPlayer {
  fn new(track_list: Vec<Handle<AudioSource>>) -> Self {
    Self { track_list }
  }
}
// This component will be attached to an entity to fade the audio in
#[derive(Component)]
struct FadeIn;

fn main() {
  App::new()
    // Enable physics
    // .add_plugins((PanOrbitCameraPlugin,))
    .insert_resource(ClearColor(Color::from(BLUE_200)))
    .add_plugins((
      // AssetPlugin::default(),
      // AudioPlugin::default(),
      // LogDiagnosticsPlugin::default(),
      DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
          position: WindowPosition::At(IVec2::new(1200, 0)),
          // title: "Bevy Game".to_string(),
          resolution: WindowResolution::new(
            // WP_W / WP_SCALE,
            // WP_H / WP_SCALE
            USE_WIN_SIZE.x,
            USE_WIN_SIZE.y
          ).with_scale_factor_override(1.0),
          present_mode: AutoNoVsync,
          // mode: Fullscreen(MonitorSelection::Primary),
          // mode: BorderlessFullscreen(MonitorSelection::Primary),
          // resizable: false,
          // fit_canvas_to_parent: true,
          // fullsize_content_view: true,
          ..Default::default()
        }),
        ..Default::default()
      }), // .set(WindowPlugin {}),
      PhysicsPlugins::default(),
      PhysicsDebugPlugin::default(),
      cubes::CubesPlugin,
      debug::DebugPlugin,
      camera::CameraPlugin,
      lights::MLightsPlugin,
      terrain::MTerrainPlugin,
      entities::base::MEntityBasePlugin,
      entities::with_children::MEntityWithChildrenPlugin,
    ))
    .insert_gizmo_config(
      PhysicsGizmos {
        aabb_color: Some(Color::WHITE),
        ..default()
      },
      GizmoConfig::default()
    )
    .add_systems(Startup, setup)
    .add_systems(Update, update)
    // .add_systems(Startup, play)
    .insert_resource(Gravity(Vec3::NEG_Y * GRAVITY))
    .run();
}

// #[derive(Component, Debug, PartialEq, Eq)]
// pub struct Terrain;

// https://github.com/bevyengine/bevy/blob/v0.15.0/examples/audio/soundtrack.rs

// fn play(
//   mut commands: Commands,
//   soundtrack_player: Res<SoundtrackPlayer>,
//   soundtrack: Query<Entity, With<AudioSink>>
//   // game_state: Res<GameState>
// ) {
//   // commands.spawn((
//   //   AudioPlayer(soundtrack_player.track_list.first().unwrap().clone()),
//   //   // AudioPlayer(track_list.first().unwrap().clone()),
//   //   PlaybackSettings {
//   //     mode: bevy::audio::PlaybackMode::Loop,
//   //     volume: bevy::audio::Volume::default(),
//   //     ..default()
//   //   },
//   //   // FadeIn,
//   // ));
// }

fn load_base_texture(asset_server: Res<AssetServer>, path: &str) -> Handle<Image> {
  let texture_handle: Handle<Image> = asset_server.load_with_settings(
    path, // "textures/terrain/base/sand.01.png",
    |s: &mut _| {
      *s = ImageLoaderSettings {
        sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
          // rewriting mode to repeat image,
          // address_mode_u: ImageAddressMode::Repeat,
          // address_mode_v: ImageAddressMode::Repeat,
          address_mode_u: ImageAddressMode::Repeat,
          address_mode_v: ImageAddressMode::Repeat,
          // address_mode_w: ImageAddressMode::ClampToBorder,
          mag_filter: ImageFilterMode::Linear,
          ..default()
        }),
        ..default()
      };
    }
  );

  texture_handle
}

fn get_base_texture_material(
  asset_server: Res<AssetServer>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  path: &str
) -> (Handle<Image>, StandardMaterial, Handle<StandardMaterial>) {
  let texture: Handle<Image> = load_base_texture(asset_server, "textures/terrain/base/sand.01.png");

  let mut material = StandardMaterial {
    // base_color: Color::BLACK,
    base_color_texture: Some(texture.clone()),
    // https://bevyengine.org/examples/assets/repeated-texture/
    // uv_transform: Affine2::from_scale(Vec2::new(1.0, 1.0)),
    // uv_transform: Affine2::from_scale(Vec2::new(2.0, 2.0)),
    // base_color: Color::srgba_u8(128, 197, 222,120),
    base_color: Color::srgba_u8(128, 197, 222, 17),
    // alpha_mode: AlphaMode::Mask(0.75),
    alpha_mode: AlphaMode::Blend,
    unlit: false,
    emissive: LinearRgba::BLACK,
    // emissive_exposure_weight: 1.0,
    perceptual_roughness: 0.85,
    // metallic: 0.0,
    reflectance: 0.05,
    // ior: 1.47,
    ..default()
  };

  // material.base_color = Color::srgba_u8(128, 197, 222, 17);

  // material.base_color_tiling = Vec2::new(2.0, 2.0); // Scale the texture UVs
  let handle = materials.add(material.clone());

  (texture, material, handle)
}

// prettier-ignore
// static T2: [[i8; 13]; 13] = [
//   [ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
//   [ 0,16,16,16,16,16,16,16,16,16,16,16, 0 ],
//   [ 0,16, 8, 8, 8, 8, 8, 8, 8, 8, 8,16, 0 ],
//   [ 0,16, 8, 4, 4, 4, 4, 4, 4, 4, 8,16, 0 ],
//   [ 0,16, 8, 4, 2, 2, 2, 2, 2, 4, 8,16, 0 ],
//   [ 0,16, 8, 4, 2, 1, 1, 1, 2, 4, 8,16, 0 ],
//   [ 0,16, 8, 4, 2, 1, 1, 1, 2, 4, 8,16, 0 ],
//   [ 0,16, 8, 4, 2, 1, 1, 1, 2, 4, 8,16, 0 ],
//   [ 0,16, 8, 4, 2, 2, 2, 2, 2, 4, 8,16, 0 ],
//   [ 0,16, 8, 4, 4, 4, 4, 4, 4, 4, 8,16, 0 ],
//   [ 0,16, 8, 8, 8, 8, 8, 8, 8, 8, 8,16, 0 ],
//   [ 0,16,16,16,16,16,16,16,16,16,16,16, 0 ],
//   [ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
// ];

// prettier-ignore
// static T2: [[i8; 13]; 13] = [
//   [0,0,0,0,0,0,0,0,0,0,0,0,0],
//   [0,0,0,0,0,0,0,0,0,0,0,0,0],
//   [0,0,8,8,8,8,8,8,8,8,8,0,0],
//   [0,0,8,4,4,4,4,4,4,4,8,0,0],
//   [0,0,8,4,2,2,2,2,2,4,8,0,0],
//   [0,0,8,4,2,1,1,1,2,4,8,0,0],
//   [0,0,8,4,2,1,1,1,2,4,8,0,0],
//   [0,0,8,4,2,1,1,1,2,4,8,0,0],
//   [0,0,8,4,2,2,2,2,2,4,8,0,0],
//   [0,0,8,4,4,4,4,4,4,4,8,0,0],
//   [0,0,8,8,8,8,8,8,8,8,8,0,0],
//   [0,0,0,0,0,0,0,0,0,0,0,0,0],
//   [0,0,0,0,0,0,0,0,0,0,0,0,0],
// ];

// prettier-ignore
// static T2: [[i8; 13]; 13] = [
//   [32,32,32,32,32,32,32,32,32,32,32,32,32],
//   [32,32,32,32,32,32,32,32,32,32,32,32,32],
//   [32,32,16,16,16,16,16,16,16,16,16,32,32],
//   [32,32,16, 8, 8, 8, 8, 8, 8, 8,16,32,32],
//   [32,32,16, 8, 4, 4, 4, 4, 4, 8,16,32,32],
//   [32,32,16, 8, 4, 2, 2, 2, 4, 8,16,32,32],
//   [32,32,16, 8, 4, 2, 1, 2, 4, 8,16,32,32],
//   [32,32,16, 8, 4, 2, 2, 2, 4, 8,16,32,32],
//   [32,32,16, 8, 4, 4, 4, 4, 4, 8,16,32,32],
//   [32,32,16, 8, 8, 8, 8, 8, 8, 8,16,32,32],
//   [32,32,16,16,16,16,16,16,16,16,16,32,32],
//   [32,32,32,32,32,32,32,32,32,32,32,32,32],
//   [32,32,32,32,32,32,32,32,32,32,32,32,32],
// ];

// // prettier-ignore
// static T2: [[i8; 13]; 13] = [
//   [64,64,64,64,64,64,64,64,64,64,64,64,64],
//   [64,64,64,64,64,64,64,64,64,64,64,64,64],
//   [64,64,32,32,32,32,64,32,32,32,32,64,64],
//   [64,64,32,32,16,16,32,16,16,32,32,64,64],
//   [64,64,32,16, 8, 4, 8, 4, 8,16,32,64,64],
//   [64,64,32,16, 4, 1, 1, 1, 4,16,32,64,64],
//   [64,64,64,32, 8, 1, 1, 1, 8,32,64,64,64],
//   [64,64,32,16, 4, 1, 1, 1, 4,16,32,64,64],
//   [64,64,32,16, 8, 4, 8, 4, 8,16,32,64,64],
//   [64,64,32,32,16,16,32,16,16,32,32,64,64],
//   [64,64,32,32,32,32,64,32,32,32,32,64,64],
//   [64,64,64,64,64,64,64,64,64,64,64,64,64],
//   [64,64,64,64,64,64,64,64,64,64,64,64,64],
// ];

// prettier-ignore
static T2: [[i16; 13]; 13] = [
  [512,512,512,512,512,512,512,512,512,512,512,512,512],
  [512,256,256,256,256,256,256,256,256,256,256,256,512],
  [512,256,128,128,128,128,128,128,128,128,128,256,512],
  [512,256,128, 64, 64, 64, 64, 64, 64, 64,128,256,512],
  [512,256,128, 64, 32, 32, 32, 32, 32, 64,128,256,512],
  [512,256,128, 64, 32,  4,  4,  4, 32, 64,128,256,512],
  [512,256,128, 64, 32,  4,  1,  4, 32, 64,128,256,512],
  [512,256,128, 64, 32,  4,  4,  4, 32, 64,128,256,512],
  [512,256,128, 64, 32, 32, 32, 32, 32, 64,128,256,512],
  [512,256,128, 64, 64, 64, 64, 64, 64, 64,128,256,512],
  [512,256,128,128,128,128,128,128,128,128,128,256,512],
  [512,256,256,256,256,256,256,256,256,256,256,256,512],
  [512,512,512,512,512,512,512,512,512,512,512,512,512],
];

// prettier-ignore
fn setup(
  asset_server: Res<AssetServer>,
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>
) {

  let yl = T2.len();
  let xl = T2[0].len();
  println!("{:?} {:?}", yl, xl);

  for uy in 0..yl{

    // println!("{:?}", V[uy]);
    for ux in 0..xl{
      let v = T2[uy][ux];
      print!("{} ", v);
    }
    println!("");
  }

  // return;
  
  // {
  //   // let track_1 = asset_server.load::<AudioSource>("sounds/test.01.mp3");
  //   // let track_2 = asset_server.load::<AudioSource>("sounds/test.01.mp3");
  //   let track_1: Handle<AudioSource> = asset_server.load::<AudioSource>("sounds/test.02.ogg");
  //   // let track_1: Handle<AudioSource> = asset_server.load::<AudioSource>("sounds/paintball_shoot.01.ogg");
  //   // let track_list = vec![track_1, track_2];
  //   // commands.insert_resource(SoundtrackPlayer::new(track_list));
    
  //   commands.spawn((
  //     // AudioPlayer(soundtrack_player.track_list.first().unwrap().clone()),
  //     AudioPlayer(track_1),
  //     // AudioPlayer(track_list.first().unwrap().clone()),
  //     PlaybackSettings {
  //       mode: bevy::audio::PlaybackMode::Loop,
  //       volume: bevy::audio::Volume::default(),
  //       ..default()
  //     },
  //     // FadeIn,
  //   ));

  //   // commands.spawn(AudioPlayerS::new(
  //   //   asset_server.load("sounds/test.01.mp3"),
  //   // ));

  //   // let loader: Handle<_> = asset_server.load("sounds/test.01.mp3");
  //   // let audio  = AudioPlayer::new(loader);
  //   // commands.spawn(audio);
  // }

  // let Ok(entity) = query.get_single_mut() else { return; };

  let terrain_texture_handle: Handle<Image> = load_base_texture(asset_server, "textures/terrain/base/sand.01.png");
  // let tree_platanus_texture_handle: Handle<Image> = load_base_texture(asset_server, "textures/tree/platanus-acerifolia-02.png");

  let terrain_material = StandardMaterial {
    // base_color: Color::BLACK,
    base_color_texture: Some(terrain_texture_handle.clone()),
    // https://bevyengine.org/examples/assets/repeated-texture/
    // uv_transform: Affine2::from_scale(Vec2::new(1.0, 1.0)),
    // uv_transform: Affine2::from_scale(Vec2::new(2.0, 2.0)),
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

  // material.base_color_tiling = Vec2::new(2.0, 2.0); // Scale the texture UVs
  let terrain_material_handle = materials.add(terrain_material);

  let mut _min: f32 = f32::MAX;
  let mut _max: f32 = -f32::MAX;

  let segments:i32 = 0;
  for z in -segments..=segments {
    for x in -segments..=segments {


      let on_z = ((T2.len() as i32) - 7 + z) as usize;
      let on_x = ((T2.len() as i32) - 7 + x) as usize;
      let dyn_scale = T2[ on_z ][ on_x ];

      let (terrain, min, max) = generate_chunk(x as f64, z as f64, dyn_scale);

      _min = if _min > min { min } else { _min };
      _max = if _max < max { max } else { _max };

      commands.spawn((
        RigidBody::Static,
        CollisionMargin(COLLISION_MARGIN),
        Collider::trimesh_from_mesh(&terrain).unwrap(),
        get_defaul_physic_debug_params(),
        Mesh3d(meshes.add(terrain)),
        MeshMaterial3d(terrain_material_handle.clone()),
        // MeshMaterial3d(materials.add(Color::srgb_u8(10, 255, 127))),
        // MeshMaterial3d(
          //   materials.add(StandardMaterial {
        //     base_color: Color::WHITE,
        //     perceptual_roughness: 0.9,
        //     ..default()
        //   })
        // ),
        // Transform::from_translation(Vec3::new(-200., 0., 0.)),
        MTerrainMarker,
        PhysicsStaticObject,
        PhysicsStaticObjectTerrain,
        AnyObject,
        Name::new("terrain_t"),
      ));

      let mut water = Mesh::from(Cuboid::new(TERRAIN_CHUNK_X, 0.01, TERRAIN_CHUNK_Z));

      if
        let Some(VertexAttributeValues::Float32x2(ref mut uvs)) = water.attribute_mut(
          Mesh::ATTRIBUTE_UV_0
        )
      {
        for uv in uvs.iter_mut() {
          uv[0] *= 32.0; // Scale U
          uv[1] *= 32.0; // Scale V
        }
      }

      // commands.spawn((
      //   // RigidBody::Static,
      //   Collider::trimesh_from_mesh(&water).unwrap(),
      //   // Sensor,
      //   // Transform::from_translation(
      //   //   Vec3::new(
      //   //   (x * TERRAIN_CHUNK_X as i32) as f32, 
      //   //   10.125, 
      //   //   (z * TERRAIN_CHUNK_Z as i32) as f32
      //   //   )
      //   // ),
      //   Transform::from_xyz(
      //     (x * TERRAIN_CHUNK_X as i32) as f32, 
      //     -13.0, 
      //     (z * TERRAIN_CHUNK_Z as i32) as f32
      //     // .looking_at(Vec3::ZERO, Vec3::ZERO)
      //   ),
      //   Mesh3d(meshes.add(water)),
      //   // MeshMaterial3d(materials.add(Color::srgb_u8(255, 40, 40))),
      //   MeshMaterial3d(materials.add(Color::srgba_u8(128, 197, 222,17))),
      //   // MeshMaterial3d(water_material_handle.clone()),
      //   // AngularVelocity(Vec3::new(2.5, 3.5, 1.5)),
      //   AnyObject,
      //   Name::new("water_t"),
      // ));
      
    }
  }

  // terrain: (min: -74.509224 max: 70.95005)

  println!("terrain: (min: {_min} max: {_max})");

}

static mut M_X: i32 = -10_000_000;
static mut M_Y: i32 = -10_000_000;

// prettier-ignore
fn update(
  // mut q_terrain: Query<&mut Transform, (With<MTerrainMarker>, Without<CameraMarker>)>,
  q_name: Query<&Name>,
  mut commands: Commands,
  mut q_player: Query<&mut Transform, (With<CameraParentMarker>, Without<MTerrainMarker>)>,
  mut q_terrain: Query<
    (Entity, &mut RigidBody, &mut Transform),
    (With<MTerrainMarker>, Without<CameraMarker>)
  >,
) {


  // let (entity, body, mut transform) = q_terrain.single_mut();
  // let terrain_t = q_name.get(entity).unwrap_or(&Name::new("unknown_t")).to_string();
  // println!("terrain_t: {:?}", terrain_t);
  // commands.entity(entity).despawn();

  // let o_player = q_player.single_mut();
  // let pos = o_player.translation;
  // let m_x = ( (pos.x + (TERRAIN_CHUNK_X / 2.0)) / TERRAIN_CHUNK_X) as i32;
  // let m_z = ( (pos.z + (TERRAIN_CHUNK_Z / 2.0)) / TERRAIN_CHUNK_Z) as i32;

  // unsafe {
  // // println!("player @: => {TERRAIN_CHUNK_X} => +>  (x: {m_x} z: {m_z} => p x/z => {}/{}", pos.x, pos.z);      

  //   if m_x != M_X || m_z != M_Y {
  //     M_X = m_x;
  //     M_Y = m_z;
  //     // println!("player @: (x: {m_x} y: {m_z})");      
  //     println!("player @: => {TERRAIN_CHUNK_X} => +>  (x: {m_x} z: {m_z} => p x/z => {}/{}", pos.x, pos.z);      
  //   }
  // }

}

// prettier-ignore
fn generate_chunk( x: f64, z: f64, dyn_scale: i16 ) -> (Mesh, f32, f32) {
  
  let noise: BasicMulti<Perlin> = BasicMulti::<Perlin>::default();
  let mut terrain = Mesh::from(
    Plane3d::default()
      .mesh()
      .size(TERRAIN_CHUNK_X, TERRAIN_CHUNK_Z)
      .subdivisions(TERRAIN_CHUNK_SUBDIVISIONS / (dyn_scale as u32))
  );

  let use_segment_separator = true;
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
      pos[1] *= 1.0;
    }

    let colors: Vec<[f32; 4]> = positions
      .iter()
      .map(|[_, g, _]| {
        let g: f32 = (*g + TERRAIN_HEIGHT) / (TERRAIN_HEIGHT * 2.0); //  * 2.0 + 2.0; // * 26.0;
        min = if min > g { g } else { min };
        max = if max < g { g } else { max };
        return terrain_cal_color_on_g(g);
      })
      .collect();
    terrain.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    terrain.compute_normals();

    if let Some(VertexAttributeValues::Float32x3(positions)) = terrain.attribute_mut(Mesh::ATTRIBUTE_POSITION ){
      for pos in positions.iter_mut() {
        pos[1] -= (((dyn_scale.abs() as f32) - 4.0) * 1.0) / 2.0;
      }
    }

  }

  if
    let Some(VertexAttributeValues::Float32x2(ref mut uvs)) = terrain.attribute_mut(
      Mesh::ATTRIBUTE_UV_0
    )
  {
    for uv in uvs.iter_mut() {
      uv[0] *= 8.0; // Scale U
      uv[1] *= 8.0; // Scale V
    }
  }

  return (terrain, min, max);
}

// prettier-ignore
fn terrain_cal_color_on_g(g: f32) -> [f32; 4] {

  let mut color: [f32; 4];

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
    color = Color::from(GREEN_500).to_linear().to_f32_array();
  } else if g > MAX_TERRAIN_H_FOR_COLOR - TERRAIN_H_COLOR_STEP * 6.2 {
    color = Color::from(GREEN_200).to_linear().to_f32_array();
  } else if g > MAX_TERRAIN_H_FOR_COLOR - TERRAIN_H_COLOR_STEP * 6.5 {
    color = Color::from(GREEN_100).to_linear().to_f32_array();
  } else if g > MAX_TERRAIN_H_FOR_COLOR - TERRAIN_H_COLOR_STEP * 7.3 { // water-upper border
    color = Color::from(GRAY_300).to_linear().to_f32_array();
  } else if g > MAX_TERRAIN_H_FOR_COLOR - TERRAIN_H_COLOR_STEP * 7.4 { // water-lower border
    color = Color::from(GRAY_300).to_linear().to_f32_array();
  } else if g > MAX_TERRAIN_H_FOR_COLOR - TERRAIN_H_COLOR_STEP * 7.5 {
    color = Color::from(GRAY_200).to_linear().to_f32_array();
  } else if g > MAX_TERRAIN_H_FOR_COLOR - TERRAIN_H_COLOR_STEP * 7.6 {
    color = Color::from(GRAY_600).to_linear().to_f32_array();
  } else if g > MAX_TERRAIN_H_FOR_COLOR - TERRAIN_H_COLOR_STEP * 8.0 {
    color = Color::from(BLUE_500).to_linear().to_f32_array();
  } else if g > MAX_TERRAIN_H_FOR_COLOR - TERRAIN_H_COLOR_STEP * 8.5 {
    color = Color::from(BLUE_600).to_linear().to_f32_array();
  } else if g > MAX_TERRAIN_H_FOR_COLOR - TERRAIN_H_COLOR_STEP * 9.0 {
    color = Color::from(BLUE_700).to_linear().to_f32_array();
  } else {
    color = Color::from(BLUE_800).to_linear().to_f32_array();
  }

  return color;
}
