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

// const TERRAIN_XZ_TO_Y_SCALLER: f32 = 8.0;
// const TERRAIN_HEIGHT: f32 = 70.0;
// const TERRAIN_CHUNK_W: f32 = 1024.0 / TERRAIN_XZ_TO_Y_SCALLER;
// const TERRAIN_CHUNK_H: f32 = 1024.0 / TERRAIN_XZ_TO_Y_SCALLER;
// const TERRAIN_CHUNK_SUBDIVISIONS_SPLIT: u32 = 32 / 4;
// const TERRAIN_CHUNK_SCALLER: f64 = 300.0;
// // prettier-ignore
// const TERRAIN_CHUNK_SUBDIVISIONS: u32 = (TERRAIN_CHUNK_SUBDIVISIONS_SPLIT / (TERRAIN_XZ_TO_Y_SCALLER as u32)) * 2;

const TERRAIN_XZ_TO_Y_SCALLER: f32 = 4.0; // 4.0;
const TERRAIN_HEIGHT: f32 = 70.0 * 2.0; // 70.0 * 2.0
const TERRAIN_CHUNK_W: f32 = (1024.0 / TERRAIN_XZ_TO_Y_SCALLER) * 4.0; // 4.0
const TERRAIN_CHUNK_H: f32 = (1024.0 / TERRAIN_XZ_TO_Y_SCALLER) * 4.0; // 4.0
const TERRAIN_CHUNK_SUBDIVISIONS_SPLIT: u32 = 32; // 32
const TERRAIN_CHUNK_SCALLER: f64 = 1000.0; // 3à0.0
// prettier-ignore
// const TERRAIN_CHUNK_SUBDIVISIONS: u32 = (TERRAIN_CHUNK_SUBDIVISIONS_SPLIT / (TERRAIN_XZ_TO_Y_SCALLER as u32)) * 1;
const TERRAIN_CHUNK_SUBDIVISIONS: u32 = 16 * 8; // 16 * 8

const WINDOW_POSITIONS_DEV_SIDE_33_PERCENT: Vec2 = Vec2::new(800.0, 1100.0);
const WINDOW_POSITIONS_DEV_SIDE_50_PERCENT: Vec2 = Vec2::new(950.0, 1100.0);

static USE_WIN_SIZE: Vec2 = WINDOW_POSITIONS_DEV_SIDE_50_PERCENT;

fn main() {
  App::new()
    // Enable physics
    // .add_plugins((PanOrbitCameraPlugin,))
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
          // present_mode: AutoNoVsync,
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

fn update() {}
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
fn setup(
  asset_server: Res<AssetServer>,
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>
) {

  // let track_1 = asset_server.load::<AudioSource>("sounds/test.01.mp3");
  // let track_2 = asset_server.load::<AudioSource>("sounds/test.01.mp3");

  let track_1: Handle<AudioSource> = asset_server.load::<AudioSource>("sounds/test.02.ogg");
  // let track_1: Handle<AudioSource> = asset_server.load::<AudioSource>("sounds/paintball_shoot.01.ogg");
  // let track_list = vec![track_1, track_2];
  // commands.insert_resource(SoundtrackPlayer::new(track_list));
  
  commands.spawn((
    // AudioPlayer(soundtrack_player.track_list.first().unwrap().clone()),
    AudioPlayer(track_1),
    // AudioPlayer(track_list.first().unwrap().clone()),
    PlaybackSettings {
      mode: bevy::audio::PlaybackMode::Loop,
      volume: bevy::audio::Volume::default(),
      ..default()
    },
    // FadeIn,
  ));

  // commands.spawn(AudioPlayerS::new(
  //   asset_server.load("sounds/test.01.mp3"),
  // ));

  // let loader: Handle<_> = asset_server.load("sounds/test.01.mp3");
  // let audio  = AudioPlayer::new(loader);
  // commands.spawn(audio);

  // let Ok(entity) = query.get_single_mut() else { return; };

  let terrain_texture_handle: Handle<Image> = load_base_texture(asset_server, "textures/terrain/base/sand.01.png");
    

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

  let segments:i32 = 1;
  for x in -segments..=segments {
    for z in -segments..=segments {
      let (terrain, min, max) = generate_chunk(x as f64, z as f64);

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
        // Terrain,
        PhysicsStaticObject,
        PhysicsStaticObjectTerrain,
        AnyObject,
        Name::new("terrain_t"),
      ));

      let mut water = Mesh::from(Cuboid::new(TERRAIN_CHUNK_W, 0.01, TERRAIN_CHUNK_H));

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

      commands.spawn((
        // RigidBody::Static,
        Collider::trimesh_from_mesh(&water).unwrap(),
        Sensor,
        // Transform::from_translation(
        //   Vec3::new(
        //   (x * TERRAIN_CHUNK_W as i32) as f32, 
        //   10.125, 
        //   (z * TERRAIN_CHUNK_H as i32) as f32
        //   )
        // ),
        Transform::from_xyz(
          (x * TERRAIN_CHUNK_W as i32) as f32, 
          -13.0, 
          (z * TERRAIN_CHUNK_H as i32) as f32
          // .looking_at(Vec3::ZERO, Vec3::ZERO)
        ),
        Mesh3d(meshes.add(water)),
        // MeshMaterial3d(materials.add(Color::srgb_u8(255, 40, 40))),
        MeshMaterial3d(materials.add(Color::srgba_u8(128, 197, 222,17))),
        // MeshMaterial3d(water_material_handle.clone()),
        // AngularVelocity(Vec3::new(2.5, 3.5, 1.5)),
        AnyObject,
        Name::new("water_t"),
      ));
    }
  }

  // terrain: (min: -74.509224 max: 70.95005)

  println!("terrain: (min: {_min} max: {_max})");

}

fn generate_chunk(
  // mut commands: Commands,
  // mut meshes: ResMut<Assets<Mesh>>,
  // mut materials: ResMut<Assets<StandardMaterial>>,
  x: f64,
  z: f64
) -> (Mesh, f32, f32) {
  let noise: BasicMulti<Perlin> = BasicMulti::<Perlin>::default();

  let mut terrain = Mesh::from(
    Plane3d::default()
      .mesh()
      // .size(1000.0, 1000.0)
      // .subdivisions(20),
      .size(TERRAIN_CHUNK_W, TERRAIN_CHUNK_H)
      .subdivisions(TERRAIN_CHUNK_SUBDIVISIONS)
  );

  let use_segment_separator = false;
  let mut min: f32 = f32::MAX;
  let mut max: f32 = -f32::MAX;

  if
    let Some(VertexAttributeValues::Float32x3(positions)) = terrain.attribute_mut(
      Mesh::ATTRIBUTE_POSITION
    )
  {
    // main terrain topology
    for pos in positions.iter_mut() {
      let xi: f32 = noise.get([
        (((pos[0] as f64) + (TERRAIN_CHUNK_W as f64) * x) as f64) / TERRAIN_CHUNK_SCALLER,
        (((pos[2] as f64) + (TERRAIN_CHUNK_H as f64) * z) as f64) / TERRAIN_CHUNK_SCALLER,
        0.0 as f64,
      ]) as f32;
      pos[0] += (TERRAIN_CHUNK_W * (x as f32)) as f32; // + ((x / 1.0) as f32);
      pos[1] = xi * TERRAIN_HEIGHT * 1.0;
      // pos[1] = 0.0;
      pos[2] += (TERRAIN_CHUNK_H * (z as f32)) as f32; // + ((z / 1.0) as f32);
      if use_segment_separator {
        pos[0] += (x / 1.0) as f32;
        pos[2] += (z / 1.0) as f32;
      }
    }

    // seconds pass
    for pos in positions.iter_mut() {
      let xi: f32 = noise.get([
        (((pos[0] as f64) + (TERRAIN_CHUNK_W as f64) * x) as f64) / (TERRAIN_CHUNK_SCALLER / 100.0),
        (((pos[2] as f64) + (TERRAIN_CHUNK_H as f64) * z) as f64) / (TERRAIN_CHUNK_SCALLER / 100.0),
        0.0 as f64,
      ]) as f32;
      pos[1] += xi * TERRAIN_HEIGHT * 0.001;
    }

    for pos in positions.iter_mut() {
      pos[1] *= 1.0;
    }

    let colors: Vec<[f32; 4]> = positions
      .iter()
      .map(|[_, g, _]| {
        // min = if min > *g { *g } else { min };
        // max = if max < *g { *g } else { max };

        // return Color::from(GREEN_400).to_linear().to_f32_array();
        // return Color::BLACK.to_linear().to_f32_array();

        // (2.6873593 - 0.5470822) / 16 == 21619375
        // max: 2.6873593
        // min: 0.5470822

        const MAX: f32 = 0.7336247; // 0.75;
        const MIN: f32 = 0.28396824; // 0.25;

        let step = (MAX - MIN) / 12.0;
        // let M = 1.0;

        let g = (*g + TERRAIN_HEIGHT) / (TERRAIN_HEIGHT * 2.0); //  * 2.0 + 2.0; // * 26.0;
        // println!("{:?}", g);
        // min: 0.25 => max: 0.75

        min = if min > g { g } else { min };
        max = if max < g { g } else { max };

        if g > MAX - step * 2.0 {
          return Color::from(GRAY_100).to_linear().to_f32_array();
        }

        if g > MAX - step * 2.1 {
          return Color::from(GRAY_200).to_linear().to_f32_array();
        }

        if g > MAX - step * 2.2 {
          return Color::from(GRAY_300).to_linear().to_f32_array();
        }

        if g > MAX - step * 2.3 {
          return Color::from(GRAY_300).to_linear().to_f32_array();
        }

        if g > MAX - step * 2.4 {
          return Color::from(GRAY_400).to_linear().to_f32_array();
        }

        if g > MAX - step * 2.5 {
          return Color::from(GRAY_400).to_linear().to_f32_array();
        }

        if g > MAX - step * 3.0 {
          // return Color::from(RED_500).to_linear().to_f32_array();
          return Color::from(GRAY_500).to_linear().to_f32_array();
        }

        if g > MAX - step * 5.0 {
          return Color::from(GRAY_500).to_linear().to_f32_array();
        }

        if g > MAX - step * 6.0 {
          // before mountens
          return Color::from(GREEN_300).to_linear().to_f32_array();
        }
        if g > MAX - step * 6.2 {
          // return Color::from(GRAY_400).to_linear().to_f32_array();
          return Color::from(GREEN_200).to_linear().to_f32_array();
          // return Color::from(RED_500).to_linear().to_f32_array();
        }

        if g > MAX - step * 6.5 {
          // return Color::from(GRAY_400).to_linear().to_f32_array();
          return Color::from(GREEN_100).to_linear().to_f32_array();
        }

        // water-upper border
        if g > MAX - step * 7.3 {
          return Color::from(GRAY_300).to_linear().to_f32_array();
          // return Color::from(RED_500).to_linear().to_f32_array();
        }

        // water-lower border
        if g > MAX - step * 7.4 {
          return Color::from(GRAY_300).to_linear().to_f32_array();
          // return Color::from(RED_500).to_linear().to_f32_array();
        }

        if g > MAX - step * 7.5 {
          return Color::from(GRAY_200).to_linear().to_f32_array();
        }

        if g > MAX - step * 7.6 {
          return Color::from(GRAY_600).to_linear().to_f32_array();
        }

        if g > MAX - step * 8.0 {
          // return Color::from(GRAY_400).to_linear().to_f32_array();
          return Color::from(BLUE_500).to_linear().to_f32_array();
        }

        if g > MAX - step * 8.5 {
          // return Color::from(GRAY_400).to_linear().to_f32_array();
          return Color::from(BLUE_600).to_linear().to_f32_array();
        }

        if g > MAX - step * 9.0 {
          // return Color::from(GRAY_400).to_linear().to_f32_array();
          return Color::from(BLUE_700).to_linear().to_f32_array();
        }

        return Color::from(BLUE_800).to_linear().to_f32_array();

        if g > MAX {
          Color::from(GRAY_100).to_linear().to_f32_array()
        } else if g > MAX - step * 1.0 {
          Color::from(GRAY_300).to_linear().to_f32_array()
        } else if g > MAX - step * 2.0 {
          Color::from(AMBER_800).to_linear().to_f32_array()
        } else if g > MAX - step * 3.0 {
          Color::from(YELLOW_400).to_linear().to_f32_array()
        } else if g > MAX - step * 4.0 {
          Color::from(YELLOW_500).to_linear().to_f32_array()
        } else if g > MAX - step * 5.0 {
          Color::from(AMBER_400).to_linear().to_f32_array()
        } else if g > MAX - step * 6.0 {
          Color::from(AMBER_500).to_linear().to_f32_array()
        } else if g > MAX - step * 7.0 {
          Color::from(AMBER_600).to_linear().to_f32_array()
        } else if g > MAX - step * 8.0 {
          Color::from(AMBER_700).to_linear().to_f32_array()
        } else if g > MAX - step * 9.0 {
          Color::from(AMBER_800).to_linear().to_f32_array()
        } else if g > MAX - step * 0.0 {
          Color::from(GREEN_800).to_linear().to_f32_array()
        } else if g > MAX - step * 10.0 {
          Color::from(ORANGE_400).to_linear().to_f32_array()
        } else if g > MAX - step * 11.0 {
          Color::from(BLUE_400).to_linear().to_f32_array()
        } else if g > MAX - step * 12.0 {
          Color::from(GRAY_800).to_linear().to_f32_array()
        } else if g > MAX - step * 13.0 {
          Color::from(PURPLE_400).to_linear().to_f32_array()
        } else {
          // Color::from(GREEN_600).to_linear().to_f32_array()
          Color::from(BLACK).to_linear().to_f32_array()
        }
      })
      .collect();
    terrain.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    terrain.compute_normals();
    // terrain.translate_by();
    // terrain.normalize_joint_weights();
    // terrain.with_computed_normals();
    // return terrain.with_computed_normals();
  }
  // println!("min: {min} max: {max}");

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
