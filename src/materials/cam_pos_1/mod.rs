use bevy::{ pbr::MaterialExtension, prelude::*, render::render_resource::* };
use bevy::reflect::*;

const MATERIAL_UP_PATH: &str = "shaders/cam-pos/main.wgsl";
const MATERIAL_DOWN_PATH: &str = "shaders/cam-pos/main.wgsl";

#[derive(Asset, AsBindGroup, Component, Reflect, Debug, Clone)]
pub struct CamPosExtension {
  #[uniform(100)]
  pub height: f32,
  #[uniform(101)]
  pub time_t: f32,
}

impl MaterialExtension for CamPosExtension {
  // fn vertex_shader() -> ShaderRef {
  //   MATERIAL_UP_PATH.into()
  // }

  fn fragment_shader() -> ShaderRef {
    MATERIAL_UP_PATH.into()
  }
}
