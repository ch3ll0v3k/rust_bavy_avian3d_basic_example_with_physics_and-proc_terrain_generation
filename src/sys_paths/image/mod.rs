// use bevy::image::Image;

pub enum EImagePaths {
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

impl EImagePaths {
  pub fn as_str(&self) -> &'static str {
    match self {
      EImagePaths::Base => "textures/terrain/base/sand.01.png",
      EImagePaths::SkyOne => "textures/sky/sky.01.png",
      EImagePaths::SkyTwo => "textures/sky/sky.02.png",
      EImagePaths::SkyBoxOne => "textures/sky/sky-box.01.png",
      EImagePaths::SkyBoxTwo => "textures/sky/sky-box.02.png",
      EImagePaths::SkyBox3 => "textures/sky/sky-box.03.png",
      EImagePaths::SkySegDown => "textures/sky/sky-segments/down.png",
      EImagePaths::SkySegEast => "textures/sky/sky-segments/east.png",
      EImagePaths::SkySegNorth => "textures/sky/sky-segments/north.png",
      EImagePaths::SkySegSouth => "textures/sky/sky-segments/south.png",
      EImagePaths::SkySegUp => "textures/sky/sky-segments/up.png",
      EImagePaths::SkySegWest => "textures/sky/sky-segments/west.png",
    }
  }
}
