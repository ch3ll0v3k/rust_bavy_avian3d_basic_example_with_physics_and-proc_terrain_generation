use bevy::{ pbr::MaterialExtension, prelude::*, render::render_resource::* };

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct WaterExtension {
  // We need to ensure that the bindings of the base material and the extension do not conflict,
  // so we start from binding slot 100, leaving slots 0-99 for the base material.
  #[uniform(100)]
  pub quantize_steps: u32,
}

const MATERIAL_PATH: &str = "shaders/water/material.wgsl";

impl MaterialExtension for WaterExtension {
  fn vertex_shader() -> ShaderRef {
    MATERIAL_PATH.into()
  }

  fn fragment_shader() -> ShaderRef {
    MATERIAL_PATH.into()
  }

  fn deferred_fragment_shader() -> ShaderRef {
    MATERIAL_PATH.into()
  }
}
