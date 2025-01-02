pub enum RockGrass {
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
impl RockGrass {
  pub fn as_str(&self) -> &'static str {
    match self {
      RockGrass::Ao => "textures/terrain/pbr/rocky_grass/rocky_terrain_02_ao_1k.png",
      RockGrass::Arm => "textures/terrain/pbr/rocky_grass/rocky_terrain_02_arm_1k.png",
      RockGrass::Diff => "textures/terrain/pbr/rocky_grass/rocky_terrain_02_diff_1k.png",
      RockGrass::DiffLight => "textures/terrain/pbr/rocky_grass/rocky_terrain_02_diff_1k_light.png",
      RockGrass::Disp => "textures/terrain/pbr/rocky_grass/rocky_terrain_02_disp_1k.png",
      RockGrass::NorGl => "textures/terrain/pbr/rocky_grass/rocky_terrain_02_nor_gl_1k.png",
      RockGrass::Rough => "textures/terrain/pbr/rocky_grass/rocky_terrain_02_rough_1k.png",
      RockGrass::RoughAo => "textures/terrain/pbr/rocky_grass/rocky_terrain_02_rough_ao_1k.png",
    }
  }
}
