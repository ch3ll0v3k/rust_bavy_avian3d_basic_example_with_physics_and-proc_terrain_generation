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
  F0CrackedSand,
  F0CrackedTreeBark,
  F0DirtySand,
  F0FantasyColoredRockStone,
  F0GreenMoss,
  F0GrayMoss,
  F0SilverMoss,
  F0IceAndSnowGround,
  F0StoneRockMossMusk,

  Walet1Base,
  Walet1Normal,
}

// prettier-ignore
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

      EImagePaths::F0CrackedSand => "textures/terrain/f0/cracked_sand-1024x1024.png",
      EImagePaths::F0CrackedTreeBark => "textures/terrain/f0/cracked_tree_bark-1024x1024.png",
      EImagePaths::F0DirtySand => "textures/terrain/f0/dirty_sand-1024x1024.png",
      EImagePaths::F0FantasyColoredRockStone => "textures/terrain/f0/fantasy_colored_rock_stone-1024x1024.png",
      EImagePaths::F0GreenMoss => "textures/terrain/f0/green_moss-1024x1024.png",
      EImagePaths::F0GrayMoss => "textures/terrain/f0/gray_moss-1024x1024.png",
      EImagePaths::F0SilverMoss => "textures/terrain/f0/silver_moss-1024x1024.png",
      EImagePaths::F0IceAndSnowGround => "textures/terrain/f0/ice_and_snow_ground-1024x1024.png",
      EImagePaths::F0StoneRockMossMusk => "textures/terrain/f0/stone_rock_moss_musk-1024x1024.png",

      EImagePaths::Walet1Base => "textures/water/water.01.base.png",
      EImagePaths::Walet1Normal => "textures/water/water.01.normal.png",

      // EImagePaths::Walet1Base => "textures/water/Water-base-0325.png",
      // EImagePaths::Walet1Normal => "textures/water/Water-normal-0325.png",


    }
  }
}
