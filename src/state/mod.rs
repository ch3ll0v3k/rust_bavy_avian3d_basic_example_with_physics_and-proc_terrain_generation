use std::time::Duration;

use avian3d::prelude::*;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy::{
  app::{ App, Plugin, Startup, Update },
  input::common_conditions::{ input_just_pressed },
  prelude::{ Commands, KeyCode, NextState, Res, ResMut, State, States },
};

// https://idanarye.github.io/bevy-tnua/avian3d/schedule/struct.Physics.html

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MGameState {
  #[default]
  Running,
  Paused,
}

pub struct MGameStatePlugin;
// {
//   pub state: MGameState,
// }

// prettier-ignore
impl Plugin for MGameStatePlugin {
  fn build(&self, app: &mut App) {
    app
      // .add_plugins(())
      // .add_plugins(MGameStatePlugin {
      //   state: MGameState::Paused,
      // })
      .insert_state(MGameState::Running)
      .add_systems(Startup, startup)
      .add_systems(Update, update)
      .add_systems(Update, toggle_pause_game.run_if(input_just_pressed(KeyCode::Escape)))
      .add_systems(Update, on_pause.run_if(in_state(MGameState::Paused)))
      .add_systems(Update, on_running.run_if(
        in_state(MGameState::Running)
      ));
  }
}

fn startup(mut commands: Commands) {}
fn update() {}

fn on_pause() {
  // dbg!("on_pause....");
}

fn on_running() {
  // dbg!("on_running....");
}

// prettier-ignore
fn toggle_pause_game(
  state: Res<State<MGameState>>,
  mut next_state: ResMut<NextState<MGameState>>,
  mut time: ResMut<Time<Physics>>
) {
  match state.get() {
    MGameState::Paused => {
      toggle_physics_state(time, !false);
      toggle_game_state(next_state, !true);
    },
    MGameState::Running => {
      toggle_physics_state(time, !true);
      toggle_game_state(next_state, !false);
    },
  }
}

fn toggle_physics_state(mut time: ResMut<Time<Physics>>, is_physics_on: bool) {
  // dbg!("toggle_physics_state: is_physics_on: {is_physics_on}");
  if is_physics_on {
    time.unpause();
  } else {
    time.pause();
  }
}

fn toggle_game_state(mut next_state: ResMut<NextState<MGameState>>, is_paused: bool) {
  // dbg!("toggle_game_state: is_paused: {is_paused}");
  if is_paused {
    next_state.set(MGameState::Paused);
  } else {
    next_state.set(MGameState::Running);
  }
}
