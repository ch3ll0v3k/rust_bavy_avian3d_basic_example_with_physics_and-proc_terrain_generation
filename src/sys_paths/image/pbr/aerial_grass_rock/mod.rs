pub enum AerialGrassRock {
  Ao,
  Arm,
  Diff,
  DiffLight,
  Disp,
  NorGl,
  Rough,
  RoughAo,
}

// prettier-ignore
impl AerialGrassRock {
  pub fn as_str(&self) -> &'static str {
    match self {

      AerialGrassRock::Ao => "textures/terrain/pbr/aerial-grass-rock/aerial_grass_rock_ao_1k.png",
      AerialGrassRock::Arm => "textures/terrain/pbr/aerial-grass-rock/aerial_grass_rock_arm_1k.png",
      AerialGrassRock::Diff => "textures/terrain/pbr/aerial-grass-rock/aerial_grass_rock_diff_1k.png",
      AerialGrassRock::DiffLight => "textures/terrain/pbr/aerial-grass-rock/aerial_grass_rock_diff_1k_light.png",
      AerialGrassRock::Disp => "textures/terrain/pbr/aerial-grass-rock/aerial_grass_rock_disp_1k.png",
      AerialGrassRock::NorGl => "textures/terrain/pbr/aerial-grass-rock/aerial_grass_rock_nor_gl_1k.png",
      AerialGrassRock::Rough => "textures/terrain/pbr/aerial-grass-rock/aerial_grass_rock_rough_1k.png",
      AerialGrassRock::RoughAo => "textures/terrain/pbr/aerial-grass-rock/aerial_grass_rock_rough_ao_1k.png",
    }
  }
}
