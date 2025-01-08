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
use wgpu::PowerPreference;

use std::{ collections::HashMap, time::Duration };

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
    App, PluginGroupBuilder, ScheduleRunnerPlugin, Startup, Update, PluginGroup
  }, 
  asset::{ AssetServer, Assets, Handle }, 
  audio::{ AudioPlayer, AudioPlugin, AudioSource, PlaybackMode, PlaybackSettings, Volume }, 
  color::{ palettes::{css::*, tailwind::*}, Color }, 
  core_pipeline::{
    core_3d::graph::{ Core3d, Node3d },
    fullscreen_vertex_shader::fullscreen_shader_vertex_state,
  }, 
  ecs::query::{ 
    QueryItem, QuerySingleError 
  }, 
  gizmos::AppGizmoBuilder, image::{ 
    ImageAddressMode, ImageFilterMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor 
  }, 
  math::{ IVec2, Vec2, Vec3 }, pbr::{ 
    CascadeShadowConfigBuilder, ExtendedMaterial, OpaqueRendererMethod, StandardMaterial 
  }, 
  prelude::{
    ClearColor, Commands, IntoSystemConfigs, Mesh, Res, ResMut, Resource, GizmoConfig
  }, 
  render::{
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
  }, 
  time::{ common_conditions::on_timer, Fixed, Time }, 
  utils::default, 
  window::WindowMode::{self, Windowed, BorderlessFullscreen}, 
  DefaultPlugins
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
    .add_systems(Startup, setup)
    .add_systems(Update, (
      update
    ).run_if(
      on_timer(Duration::from_millis(1000))
    ))
    // .add_systems(Update, update_settings)
    .run();
}

// prettier-ignore
fn setup(
  // mut res_mut_audio_cache: Option<ResMut</*res_mut_texture_cache::*/AudioCache>>,
  // asset_server: Res<AssetServer>,
  // mut commands: Commands,
  // mut meshes: ResMut<Assets<Mesh>>,
  // mut materials: ResMut<Assets<StandardMaterial>>
) {

}

// prettier-ignore
fn update() {}
