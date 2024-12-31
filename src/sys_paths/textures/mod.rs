// use bevy::image::Image;

pub enum EPaths {
  Base,
  SkyOne,
  SkyTwo,
  SkyBoxOne,
  SkyBoxTwo,
  SkyBox3,
  SkySegDown,
  SkySegEast,
  SkySegNorth,
  SkySegSouth,
  SkySegUp,
  SkySegWest,
}

impl EPaths {
  pub fn as_str(&self) -> &'static str {
    match self {
      EPaths::Base => "textures/terrain/base/sand.01.png",
      EPaths::SkyOne => "textures/sky/sky.01.png",
      EPaths::SkyTwo => "textures/sky/sky.02.png",
      EPaths::SkyBoxOne => "textures/sky/sky-box.01.png",
      EPaths::SkyBoxTwo => "textures/sky/sky-box.02.png",
      EPaths::SkyBox3 => "textures/sky/sky-box.03.png",
      EPaths::SkySegDown => "textures/sky/sky-segments/down.png",
      EPaths::SkySegEast => "textures/sky/sky-segments/east.png",
      EPaths::SkySegNorth => "textures/sky/sky-segments/north.png",
      EPaths::SkySegSouth => "textures/sky/sky-segments/south.png",
      EPaths::SkySegUp => "textures/sky/sky-segments/up.png",
      EPaths::SkySegWest => "textures/sky/sky-segments/west.png",
    }
  }
}
