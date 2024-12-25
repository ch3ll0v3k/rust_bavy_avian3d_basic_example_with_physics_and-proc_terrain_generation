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

const TERRAIN_XZ_TO_Y_SCALLER: f32 = 4.0;
const TERRAIN_HEIGHT: f32 = 70.0 * 1.2;
const TERRAIN_CHUNK_W: f32 = 1024.0 / TERRAIN_XZ_TO_Y_SCALLER;
const TERRAIN_CHUNK_H: f32 = 1024.0 / TERRAIN_XZ_TO_Y_SCALLER;
const TERRAIN_CHUNK_SUBDIVISIONS_SPLIT: u32 = 32;
const TERRAIN_CHUNK_SCALLER: f64 = 300.0;
// prettier-ignore
// const TERRAIN_CHUNK_SUBDIVISIONS: u32 = (TERRAIN_CHUNK_SUBDIVISIONS_SPLIT / (TERRAIN_XZ_TO_Y_SCALLER as u32)) * 1;
const TERRAIN_CHUNK_SUBDIVISIONS: u32 = 16;

fn main() {
  App::new()
    // Enable physics
    // .add_plugins((PanOrbitCameraPlugin,))
    .add_plugins((
      // LogDiagnosticsPlugin::default(),
      DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
          // title: "Bevy Game".to_string(),
          resolution: WindowResolution::new(
            WP_W / WP_SCALE,
            WP_H / WP_SCALE
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
    .insert_resource(Gravity(Vec3::NEG_Y * GRAVITY))
    .run();
}

// #[derive(Component, Debug, PartialEq, Eq)]
// pub struct Terrain;

fn update() {}

// prettier-ignore
fn setup(
  asset_server: Res<AssetServer>,
   mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>
) {
  
  // let Ok(entity) = query.get_single_mut() else { return; };

  let texture_handle: Handle<Image> = asset_server.load_with_settings(
    "textures/terrain/base/sand.01.png",
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
      }
    },
  );

  // let texture_handle: Handle<Image> = asset_server.load("textures/terrain/base/sand.01.png" );

  // let base_color_texture: Option<Handle<Image>> = Some(asset_server.load("earth/base_color.jpg"));

  let material = StandardMaterial {
    // base_color: Color::BLACK,
    base_color_texture: Some(texture_handle.clone()),
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
  let material_handle = materials.add(material);

  let max = 0;
  for x in -max..=max {
    for z in -max..=max {
      let terrain: Mesh = generate_chunk(x as f64, z as f64);

      commands.spawn((
        RigidBody::Static,
        CollisionMargin(COLLISION_MARGIN),
        Collider::trimesh_from_mesh(&terrain).unwrap(),
        get_defaul_physic_debug_params(),
        Mesh3d(meshes.add(terrain)),
        MeshMaterial3d(material_handle.clone()),
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
    }
  }
}

fn generate_chunk(
  // mut commands: Commands,
  // mut meshes: ResMut<Assets<Mesh>>,
  // mut materials: ResMut<Assets<StandardMaterial>>,
  x: f64,
  z: f64
) -> Mesh {
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
    // for pos in positions.iter_mut() {
    //     let xi: f32 = noise.get([
    //         pos[0] as f64 / (TERRAIN_CHUNK_SCALLER * 0.1) + (TERRAIN_CHUNK_SCALLER * x),
    //         pos[2] as f64 / (TERRAIN_CHUNK_SCALLER * 0.1) + (TERRAIN_CHUNK_SCALLER * z),
    //         0. as f64,
    //     ]) as f32;
    //     pos[1] += xi * TERRAIN_HEIGHT * 0.1 / TERRAIN_XZ_TO_Y_SCALLER;
    // }

    // third pass
    // for pos in positions.iter_mut() {
    //     let xi: f32 = noise.get([
    //         pos[0] as f64 / (TERRAIN_CHUNK_SCALLER * 0.01) + (TERRAIN_CHUNK_SCALLER * x),
    //         pos[2] as f64 / (TERRAIN_CHUNK_SCALLER * 0.01) + (TERRAIN_CHUNK_SCALLER * z),
    //         0. as f64,
    //     ]) as f32;
    //     pos[1] += xi * TERRAIN_HEIGHT * 0.1 / TERRAIN_XZ_TO_Y_SCALLER * 0.2;
    // }

    for pos in positions.iter_mut() {
      pos[1] *= 1.0;
    }

    let colors: Vec<[f32; 4]> = positions
      .iter()
      .map(|[_, g, _]| {
        // return Color::from(GREEN_400).to_linear().to_f32_array();
        // return Color::BLACK.to_linear().to_f32_array();

        // (2.6873593 - 0.5470822) / 16 == 21619375
        // max: 2.6873593
        // min: 0.5470822

        let step = 0.149;
        // let M = 1.0;

        let g = (*g / TERRAIN_HEIGHT) * 2.0 + 2.0 - 0.5; // * 26.0;
        // println!("{:?}", g);

        if g > 1.7 {
          return Color::from(GRAY_500).to_linear().to_f32_array();
        }
        return Color::from(GREEN_500).to_linear().to_f32_array();
        // if g > 0.8 {
        if g > 2.6 {
          Color::from(GRAY_100).to_linear().to_f32_array()
        } else if g > 2.6 - step * 1.0 {
          Color::from(GRAY_300).to_linear().to_f32_array()
        } else if g > 2.6 - step * 2.0 {
          Color::from(AMBER_800).to_linear().to_f32_array()
        } else if g > 2.6 - step * 3.0 {
          Color::from(YELLOW_400).to_linear().to_f32_array()
        } else if g > 2.6 - step * 4.0 {
          Color::from(YELLOW_500).to_linear().to_f32_array()
        } else if g > 2.6 - step * 5.0 {
          Color::from(AMBER_400).to_linear().to_f32_array()
        } else if g > 2.6 - step * 6.0 {
          Color::from(AMBER_500).to_linear().to_f32_array()
        } else if g > 2.6 - step * 7.0 {
          Color::from(AMBER_600).to_linear().to_f32_array()
        } else if g > 2.6 - step * 8.0 {
          Color::from(AMBER_700).to_linear().to_f32_array()
        } else if g > 2.6 - step * 9.0 {
          Color::from(AMBER_800).to_linear().to_f32_array()
        } else if g > 2.6 - step * 0.0 {
          Color::from(GREEN_800).to_linear().to_f32_array()
        } else if g > 2.6 - step * 10.0 {
          Color::from(ORANGE_400).to_linear().to_f32_array()
        } else if g > 2.6 - step * 11.0 {
          Color::from(BLUE_400).to_linear().to_f32_array()
        } else if g > 2.6 - step * 12.0 {
          Color::from(GRAY_800).to_linear().to_f32_array()
        } else if g > 2.6 - step * 13.0 {
          Color::from(PURPLE_400).to_linear().to_f32_array()
        } else {
          // Color::from(GREEN_600).to_linear().to_f32_array()
          Color::from(RED_600).to_linear().to_f32_array()
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

  if
    let Some(VertexAttributeValues::Float32x2(ref mut uvs)) = terrain.attribute_mut(
      Mesh::ATTRIBUTE_UV_0
    )
  {
    for uv in uvs.iter_mut() {
      uv[0] *= 2.0; // Scale U
      uv[1] *= 2.0; // Scale V
    }
  }

  return terrain;
}
