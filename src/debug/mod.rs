// use avian3d::prelude::*;
use avian3d::prelude::DebugRender;

use bevy::app::{ App, Plugin, Startup, Update };
use bevy::color::{ Color, palettes::tailwind::* };

use bevy_diagnostic::LogDiagnosticsPlugin;

use bevy::diagnostic::{
  Diagnostic,
  DiagnosticPath,
  DiagnosticsStore,
  EntityCountDiagnosticsPlugin,
  FrameTimeDiagnosticsPlugin,
  RegisterDiagnostic,
  SystemInformationDiagnosticsPlugin,
};

use bevy::prelude::{
  Commands,
  Component,
  Entity,
  IntoSystemConfigs,
  KeyCode,
  Query,
  Res,
  ResMut,
  Resource,
  Text,
  With,
  Without,
};

use bevy::math::Vec3;
use bevy::pbr::wireframe::{ Wireframe, WireframePlugin, WireframeConfig };
use bevy::text::TextColor;
use bevy::time::{ Fixed, Time };
use bevy::input::{ ButtonInput, common_conditions::input_just_pressed };

use crate::dbgln;
use crate::markers::m_bevy::AnyObject;

pub const ALLOWED_DEBUG_PHYSICS: bool = !true;
pub const ALLOWED_DEBUG_ENGINE: bool = true;
pub const ALLOWED_DEBUG_FPS: bool = true;

pub const IS_WIREFRAME_DEFAULT_ON: bool = false;

const SYSTEM_ITERATION_COUNT: DiagnosticPath = DiagnosticPath::const_new("system_iteration_count");

#[derive(Component, Debug, PartialEq, Eq)]
struct NameTextMarker;

#[derive(Resource, Debug, PartialEq, Eq)]
struct IWireframeOn {
  is_on: bool,
}

pub struct DebugPlugin;

// prettier-ignore
impl Plugin for DebugPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, startup)
      .add_plugins((
        WireframePlugin,
        FrameTimeDiagnosticsPlugin,
        EntityCountDiagnosticsPlugin,
        SystemInformationDiagnosticsPlugin,
      ))
      .register_diagnostic(Diagnostic::new(SYSTEM_ITERATION_COUNT).with_suffix(" iterations"))
      .add_systems(Update, update)
      .add_systems(Update, toggle_wireframe.run_if(input_just_pressed(KeyCode::KeyL)))
      .add_systems(Update, update_fps);

    app
      // Wireframes can be configured with this resource. This can be changed at runtime.
      // .insert_resource(WireframeConfig {
      //   // The global wireframe config enables drawing of wireframes on every mesh,
      //   // except those with `NoWireframe`. Meshes with `Wireframe` will always have a wireframe,
      //   // regardless of the global configuration.
      //   global: false,
      //   default_color: Color::from(GREEN_100),
      // })
      .insert_resource(IWireframeOn { is_on: IS_WIREFRAME_DEFAULT_ON });


  }
}

fn startup(mut commands: Commands) {
  // prettier-ignore
  if !ALLOWED_DEBUG_FPS { return; }

  commands.spawn((Text::new("FPS: 0"), TextColor::from(RED_500), NameTextMarker));
}

fn update() {
  // for (entity, _cube_marker) in query.iter() {
  //   let mut position = cube_positions.0;
  //   position.x += time.delta_seconds();
  //   cube_positions.0 = position;
  //   commands.entity(entity).insert(position);
  // }
}

// prettier-ignore
pub fn is_allowed_debug_physics() -> bool { ALLOWED_DEBUG_PHYSICS }
// prettier-ignore
pub fn is_allowed_debug_engine() -> bool { ALLOWED_DEBUG_ENGINE }
// prettier-ignore
pub fn is_allowed_debug_fps() -> bool { ALLOWED_DEBUG_FPS }

pub fn get_defaul_physic_debug_params() -> DebugRender {
  if ALLOWED_DEBUG_PHYSICS {
    DebugRender::default()
      .with_collider_color(Color::srgb(1.0, 255.0, 1.0))
      .with_axes(Vec3::new(2.0, 2.0, 2.0))
      .with_aabb_color(Color::srgb(255.0, 0.0, 0.0))
  } else {
    DebugRender::none()
  }
}

fn update_fps(
  mut names: Query<&mut Text, With<NameTextMarker>>,
  diagnostics: Res<DiagnosticsStore>,
  fixed_time: Res<Time<Fixed>>
) {
  // prettier-ignore
  if !ALLOWED_DEBUG_FPS { return; }

  if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
    let mut writer = names.single_mut();
    if let Some(raw) = fps.smoothed() {
      let s: String = format!("FPS: {:.2}", raw);
      writer.0 = s.to_string();
      // *writer.text(text, 4) = format!("{raw:.2}");
    }
  }
}

fn toggle_wireframe(
  mut is_wireframe_on: ResMut<IWireframeOn>,
  mut commands: Commands,
  all_allowed_entities: Query<Entity, With<AnyObject>>,
  all_wireframes: Query<Entity, (With<AnyObject>, With<Wireframe>)>,
  all_no_wireframe: Query<Entity, (With<AnyObject>, Without<Wireframe>)>,
  input: Res<ButtonInput<KeyCode>>
) {
  // prettier-ignore
  if !ALLOWED_DEBUG_ENGINE { return; }

  is_wireframe_on.is_on = !is_wireframe_on.is_on;
  dbgln!("is_wireframe_on: {}", is_wireframe_on.is_on);

  if is_wireframe_on.is_on {
    for entity in &all_allowed_entities {
      commands.entity(entity).insert(Wireframe);
    }
  } else {
    for entity in &all_allowed_entities {
      commands.entity(entity).remove::<Wireframe>();
    }
  }

  // if input.just_pressed(KeyCode::KeyL) {
  //   for entity in &all_no_wireframe {
  //     commands.entity(entity).insert(Wireframe);
  //   }
  //   for entity in &all_wireframes {
  //     commands.entity(entity).remove::<Wireframe>();
  //   }
  // }
}
