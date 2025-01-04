// use bevy::image::Image;

pub enum EConfig {
  Debug,
  Window,
}

impl EConfig {
  pub fn as_str(&self) -> &'static str {
    match self {
      EConfig::Debug => "assets/config/debug.json",
      EConfig::Window => "assets/config/window.json",
    }
  }
}
