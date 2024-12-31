// use bevy::image::Image;

pub enum EPaths {
  EnvOne,
  PaintballShoot,
}

impl EPaths {
  pub fn as_str(&self) -> &'static str {
    match self {
      EPaths::EnvOne => "sounds/test.02.ogg",
      EPaths::PaintballShoot => "sounds/paintball_shoot.01.ogg",
    }
  }
}
