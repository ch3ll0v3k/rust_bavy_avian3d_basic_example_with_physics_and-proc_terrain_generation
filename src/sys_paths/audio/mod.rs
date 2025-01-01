// use bevy::image::Image;

pub enum EAudioPaths {
  EnvOne,
  PaintballShoot,
}

impl EAudioPaths {
  pub fn as_str(&self) -> &'static str {
    match self {
      EAudioPaths::EnvOne => "sounds/test.02.ogg",
      EAudioPaths::PaintballShoot => "sounds/paintball_shoot.01.ogg",
    }
  }
}
