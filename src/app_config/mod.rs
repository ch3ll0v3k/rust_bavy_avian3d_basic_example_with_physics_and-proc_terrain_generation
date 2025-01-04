use std::fs;

use bevy::app::{ App, Plugin, Startup, Update };

pub mod debug;
pub mod window;

pub fn read_file_to_string(path: &str) -> String {
  fs::read_to_string(path).expect(format!("Failed to read config file: ({})", path).as_str())
}

pub struct AppConfigPlugin;

// prettier-ignore
impl Plugin for AppConfigPlugin {

  fn build(&self, app: &mut App) {

    app
      .insert_resource(debug::config())
      .insert_resource(window::config());

    app
      .add_systems(Startup, startup)
      .add_systems(Update, update);

  }
}

// prettier-ignore
fn startup() {}

// prettier-ignore
fn update() {}
