// use bevy::image::Image;

pub enum EAudio {
  EnvOne,
  PaintballShoot,
}

impl EAudio {
  pub fn as_str(&self) -> &'static str {
    match self {
      EAudio::EnvOne => "sounds/test.02.ogg",
      EAudio::PaintballShoot => "sounds/paintball_shoot.01.ogg",
    }
  }
}
