#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unreachable_code)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_braces)]
#![allow(unused_parens)]

use app_config::window;
use bevy_render::{ settings::WgpuSettings, RenderPlugin };
use instant::Instant;
use noise::{ BasicMulti, NoiseFn, Perlin };
use state::MGameState;
use wgpu::{ core::command, PowerPreference };

use std::{ collections::HashMap, f32::consts::PI, time::Duration };

// prettier-ignore
use avian3d::{ 
  prelude::*, 
  PhysicsPlugins, debug_render::{ PhysicsDebugPlugin, DebugRender } 
};

use bevy_window::{
  MonitorSelection,
  PresentMode,
  Window,
  WindowLevel,
  WindowPlugin,
  WindowPosition,
  WindowResolution,
};

// prettier-ignore
use bevy::{
  app::{ 
    App, PluginGroup, PluginGroupBuilder, ScheduleRunnerPlugin, Startup, Update
  }, asset::{ AssetServer, Assets, Handle }, audio::{ AudioPlayer, AudioPlugin, AudioSource, PlaybackMode, PlaybackSettings, Volume }, color::{ palettes::{css::{self, *}, tailwind::*}, Color }, core_pipeline::{
    core_3d::graph::{ Core3d, Node3d },
    fullscreen_vertex_shader::fullscreen_shader_vertex_state,
  }, ecs::query::{ 
    QueryItem, QuerySingleError 
  }, gizmos::AppGizmoBuilder, image::{ 
    ImageAddressMode, ImageFilterMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor 
  }, input::common_conditions::input_just_pressed, math::{ IVec2, Vec2, Vec3 }, pbr::{ 
    CascadeShadowConfigBuilder, ExtendedMaterial, OpaqueRendererMethod, StandardMaterial 
  }, picking::{backend::ray::RayMap, mesh_picking::ray_cast::RayMeshHit}, prelude::{
    ClearColor, Commands, GizmoConfig, IntoSystemConfigs, Mesh, Res, ResMut, Resource
  }, render::{
    extract_component::{ 
      ComponentUniforms, DynamicUniformIndex, ExtractComponent, ExtractComponentPlugin, UniformComponentPlugin 
    }, 
    mesh::VertexAttributeValues, 
    render_graph::{ 
      NodeRunError, RenderGraphApp, RenderGraphContext, RenderLabel, ViewNode, ViewNodeRunner 
    }, 
    render_resource::binding_types::{ 
      sampler, texture_2d, uniform_buffer 
    }, 
    renderer::{ 
      RenderContext, RenderDevice 
    }, 
    view::ViewTarget, RenderApp
  }, time::{ common_conditions::on_timer, Fixed, Time }, utils::default, window::WindowMode::{self, BorderlessFullscreen, Windowed}, DefaultPlugins
};

#[macro_use]
mod debug_utils;

mod ambient_audio;
mod app_config;
mod camera;
mod player;
mod cubes;
mod debug;
mod lights;
mod markers;
mod constants;
mod terrain;
mod sky;
mod entities;
mod state;
mod sys_paths;
mod asset_loader;
mod m_lib;
mod materials;
mod post_processing_pipiline;

use camera::{ PlayerCameraMarker };
use player::{ PlayerMarker };
use debug::{ get_defaul_physic_debug_params };
use lights::{ MPointLightMarker, MDirLightMarker };
use markers::{ m_avian::*, m_bevy::* };
use constants::{ viewport_settings::*, physics_world::* };
use terrain::MTerrainMarker;
use m_lib::{ colors, physics };

use bevy::prelude::*;
use bevy::render::settings::{ Backends };

#[derive(Resource)]
pub struct TTargetRayInfo {
  pub pos: Vec3,
  pub dist: f32,
}

// prettier-ignore
fn main() {
  dbgln!("App stating...");

  let window_config = app_config::window::config();
  let use_win_size: Vec2 = Vec2::new(window_config.use_win_size.x, window_config.use_win_size.y);
  let window_positions: IVec2 = IVec2::new(window_config.position.x as i32, window_config.position.y as i32);
  let window_scale_factor_override: f32 = window_config.scale_factor_override;
  let present_mode: PresentMode = if window_config.use_auto_vsyn { PresentMode::AutoVsync} else { PresentMode::AutoNoVsync};
  let window_mode: WindowMode = if window_config.use_fullscreen { BorderlessFullscreen(MonitorSelection::Primary) } else { Windowed };
  let resizable: bool = window_config.resizable;

  App::new()
    .add_plugins(app_config::AppConfigPlugin)
    .insert_resource(ClearColor(
      colors::hex_to_rgb("#624e02")
    ))
    .insert_resource(Gravity(physics::get_gravity_vec3()))
    .insert_resource(TTargetRayInfo{
      pos: Vec3::new(0.0, 0.0, 0.0),
      dist: 0.0,
    })
    // .add_plugins(
    //   ScheduleRunnerPlugin::run_loop(
    //     // Run 60 times per second.
    //     Duration::from_secs_f64(1.0 / FARERATE_LIMIT)
    //     // Duration::from_secs_f64(10.0)
    //   )
    // )
    .add_plugins((
      // LogDiagnosticsPlugin::default(),
      DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
          position: WindowPosition::At(window_positions),
          // title: "Bevy Game".to_string(),
          resolution: WindowResolution::new(
            use_win_size.x,
            use_win_size.y
          ).with_scale_factor_override(window_scale_factor_override),
          present_mode,
          // mode: Fullscreen(MonitorSelection::Primary),
          // mode: BorderlessFullscreen(MonitorSelection::Primary),
          mode: window_mode,
          resizable,
          // fit_canvas_to_parent: true,
          // fullsize_content_view: true,
          ..default()
        }),
        ..default()
      }),
      MeshPickingPlugin,
      PhysicsPlugins::default(),
      camera::CameraPlugin,
      asset_loader::MAssetLoaderPlugin,
      cubes::CubesPlugin,
      debug::DebugPlugin,
      player::PlayerPlugin,
      lights::MLightsPlugin,
      terrain::MTerrainPlugin,
      sky::MSkyPlugin,
      entities::base::MEntityBasePlugin,
      entities::with_children::MEntityWithChildrenPlugin,
      state::MGameStatePlugin,
      // RenderPlugin {
      //   render_creation: WgpuSettings {
      //   // power_preference: PowerPreference::LowPower,
      //   backends: Some(Backends::VULKAN),
      //     ..default()
      //   }.into(),
      //   ..default()
      // }
    ))
    .insert_gizmo_config(
      PhysicsGizmos {
        aabb_color: Some(Color::WHITE),
        ..default()
      },
      GizmoConfig::default()
    )
    .add_plugins(PhysicsDebugPlugin::default())
    .add_systems(Startup, setup)
    .add_systems(Update, (
      bouncing_raycast
    ))
    .add_systems(Update, (
      update
    ).run_if(
      on_timer(Duration::from_millis(1000))
    ))
    // .add_systems(Update, update_settings)

      .add_systems(Update,
        (
          handle_spaw_on_ray_cast
        )
          .run_if(in_state(MGameState::Running))
          .run_if(input_just_pressed(MouseButton::Left))
      )

    .run();
}

// prettier-ignore
fn handle_spaw_on_ray_cast(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  target_ray_info: Res<TTargetRayInfo>
) {
  let mut vec3 = target_ray_info.pos.clone();
  vec3.y += 120.0;
  let side: f32 = 100.0;

  let shape = Cuboid::new(side, side, side);
  let mesh = Mesh::from(shape);

  // prettier-ignore
  let u_id_1 = commands.spawn((
    RigidBody::Dynamic,
    Collider::trimesh_from_mesh(&mesh).unwrap(),
    Mesh3d(meshes.add(mesh)),
    MeshMaterial3d(materials.add(Color::srgb_u8(255, 0, 0))),
    // MassPropertiesBundle::from_shape(&Cuboid::from_length(side), 1.0),
    Mass(1.0),
    Transform::from_translation(vec3),
  )).id();

  dbgln!("spawn: id: {u_id_1} @ {vec3}");

}

const MAX_BOUNCES: usize = 1;
const LASER_SPEED: f32 = 0.0003;

// prettier-ignore
fn bouncing_raycast(
  mut target_ray_info: Option<ResMut<TTargetRayInfo>>,
  q_player: Single<&Transform, With<PlayerMarker>>,
  q_camera: Single<&Transform, With<PlayerCameraMarker>>,
  mut ray_cast: MeshRayCast,
  mut gizmos: Gizmos,
  time: Res<Time>,
  ray_map: Res<RayMap> // The ray map stores rays cast by the cursor
) {

  let trans_a = **q_player;
  let trans_b = **q_camera;

  let ray_pos = Vec3::new(
    trans_a.translation.x,
    trans_a.translation.y + trans_b.translation.y,
    trans_a.translation.z
  );

  let forward_a = trans_a.forward().as_vec3();
  let forward_b = trans_b.up(); // .as_vec3();

  let ray_dir = Dir3::from_xyz(forward_a.x, forward_b.z, forward_a.z).unwrap(); // .normalize();

  let ray = Ray3d::new(ray_pos, ray_dir);
  gizmos.sphere(ray_pos, 0.1, Color::from(RED_500));
  bounce_ray(ray, &mut ray_cast, &mut gizmos, Color::from(BLUE_500));

  let Some((entity, hit)) = ray_cast
    .cast_ray(ray, &RayCastSettings::default())
    .first() else {
      dbgln!("No hit");
      return;
    };

  let (x, y, z) = hit.point.into();
  let dist = hit.distance;

  let mut ray_info = target_ray_info.as_mut().unwrap();
  ray_info.pos = hit.point.clone();
  ray_info.pos.y += 2.0;

  ray_info.dist = dist;

  // Cast a ray from the cursor and bounce it off of surfaces
  // for (_, ray) in ray_map.iter() {
  //   bounce_ray(*ray, &mut ray_cast, &mut gizmos, Color::from(css::GREEN));
  // }

  // // Cast an automatically moving ray and bounce it off of surfaces
  // let t = ops::cos((time.elapsed_secs() - 4.0).max(0.0) * LASER_SPEED) * PI;

  // let ray_pos = Vec3::new(ops::sin(t), ops::cos(3.0 * t) * 0.5, ops::cos(t)) * 0.5;
  // let ray_dir = Dir3::new(-ray_pos).unwrap();
  // let ray = Ray3d::new(ray_pos, ray_dir);
  // gizmos.sphere(ray_pos, 0.1, Color::WHITE);
  // bounce_ray(ray, &mut ray_cast, &mut gizmos, Color::from(css::RED));

  // // Cast a ray from the cursor and bounce it off of surfaces
  // for (_, ray) in ray_map.iter() {
  //   bounce_ray(*ray, &mut ray_cast, &mut gizmos, Color::from(css::GREEN));
  // }
}

// Bounces a ray off of surfaces `MAX_BOUNCES` times.
fn bounce_ray(mut ray: Ray3d, ray_cast: &mut MeshRayCast, gizmos: &mut Gizmos, color: Color) {
  let mut intersections = Vec::with_capacity(MAX_BOUNCES + 1);
  intersections.push((ray.origin, Color::srgb(30.0, 0.0, 0.0)));

  for i in 0..MAX_BOUNCES {
    // Cast the ray and get the first hit
    let Some((_, hit)) = ray_cast.cast_ray(ray, &RayCastSettings::default()).first() else {
      break;
    };

    // Draw the point of intersection and add it to the list
    let brightness = 1.0 + 10.0 * (1.0 - (i as f32) / (MAX_BOUNCES as f32));
    intersections.push((hit.point, Color::BLACK.mix(&color, brightness)));
    gizmos.sphere(hit.point, 0.005, Color::BLACK.mix(&color, brightness * 2.0));

    // Reflect the ray off of the surface
    ray.direction = Dir3::new(ray.direction.reflect(hit.normal)).unwrap();
    ray.origin = hit.point + ray.direction * 1e-6;
  }
  gizmos.linestrip_gradient(intersections);
}

// prettier-ignore
fn setup(
  // mut res_mut_audio_cache: Option<ResMut</*res_mut_texture_cache::*/AudioCache>>,
  // asset_server: Res<AssetServer>,
  // mut commands: Commands,
  // mut meshes: ResMut<Assets<Mesh>>,
  // mut materials: ResMut<Assets<StandardMaterial>>
) {
  // RayMap
}

// prettier-ignore
fn update(
  mut num: Local<usize>,
) {
  // *num += 1;
  // dbgln!("num: {}", *num);
}
