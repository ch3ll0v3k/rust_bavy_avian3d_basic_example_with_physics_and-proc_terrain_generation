// use bevy::image::Image;

pub mod pbr;

pub enum EImageSky {
  SkySegDown,
  SkySegEast,
  SkySegNorth,
  SkySegSouth,
  SkySegUp,
  SkySegWest,
}

// prettier-ignore
impl EImageSky {
  pub fn as_str(&self) -> &'static str {
    match self {
      EImageSky::SkySegDown => "textures/sky/sky-segments/down.png",
      EImageSky::SkySegEast => "textures/sky/sky-segments/east.png",
      EImageSky::SkySegNorth => "textures/sky/sky-segments/north.png",
      EImageSky::SkySegSouth => "textures/sky/sky-segments/south.png",
      EImageSky::SkySegUp => "textures/sky/sky-segments/up.png",
      EImageSky::SkySegWest => "textures/sky/sky-segments/west.png",
    }
  }
}

pub enum EImageTerrainBase {
  Base,
  F0CrackedSand,
  F0CrackedTreeBark,
  F0DirtySand,
  F0FantasyColoredRockStone,
  F0GreenMoss,
  F0GrayMoss,
  F0SilverMoss,
  F0IceAndSnowGround,
  F0StoneRockMossMusk,
}

// prettier-ignore
impl EImageTerrainBase {
  pub fn as_str(&self) -> &'static str {
    match self {
      EImageTerrainBase::Base => "textures/terrain/base/sand.01.png",
      EImageTerrainBase::F0CrackedSand => "textures/terrain/common/cracked_sand-1024x1024.png",
      EImageTerrainBase::F0CrackedTreeBark => "textures/terrain/common/cracked_tree_bark-1024x1024.png",
      EImageTerrainBase::F0DirtySand => "textures/terrain/common/dirty_sand-1024x1024.png",
      EImageTerrainBase::F0FantasyColoredRockStone => "textures/terrain/common/fantasy_colored_rock_stone-1024x1024.png",
      EImageTerrainBase::F0GreenMoss => "textures/terrain/common/green_moss-1024x1024.png",
      EImageTerrainBase::F0GrayMoss => "textures/terrain/common/gray_moss-1024x1024.png",
      EImageTerrainBase::F0SilverMoss => "textures/terrain/common/silver_moss-1024x1024.png",
      EImageTerrainBase::F0IceAndSnowGround => "textures/terrain/common/ice_and_snow_ground-1024x1024.png",
      EImageTerrainBase::F0StoneRockMossMusk => "textures/terrain/common/stone_rock_moss_musk-1024x1024.png",

    }
  }
}

pub enum EImageWaterBase {
  Walet1Base,
  Walet1Normal,
  Walet2Base,
  Walet2Normal,
}

impl EImageWaterBase {
  pub fn as_str(&self) -> &'static str {
    match self {
      EImageWaterBase::Walet1Base => "textures/water/water.01.base.png",
      EImageWaterBase::Walet1Normal => "textures/water/water.01.normal.png",
      EImageWaterBase::Walet2Base => "textures/water/Water-base-0325.png",
      EImageWaterBase::Walet2Normal => "textures/water/Water-normal-0325.png",
    }
  }
}
