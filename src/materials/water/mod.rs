use bevy::{ pbr::MaterialExtension, prelude::*, render::render_resource::* };
use bevy::reflect::*;

const MATERIAL_UP_PATH: &str = "shaders/water/on-material.wgsl";
const MATERIAL_DOWN_PATH: &str = "shaders/water/below-material.wgsl";

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct WaterExtension {
  // We need to ensure that the bindings of the base material and the extension do not conflict,
  // so we start from binding slot 100, leaving slots 0-99 for the base material.
  #[uniform(100)]
  pub quantize_steps: u32,
}

impl MaterialExtension for WaterExtension {
  fn vertex_shader() -> ShaderRef {
    MATERIAL_UP_PATH.into()
  }

  fn fragment_shader() -> ShaderRef {
    MATERIAL_UP_PATH.into()
  }

  fn deferred_fragment_shader() -> ShaderRef {
    MATERIAL_UP_PATH.into()
  }
}

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct UnderWaterExtention {
  #[uniform(101)]
  pub fog_height: f32,
  #[uniform(102)]
  pub fog_color: Vec4,
  #[uniform(103)]
  pub base_color: Vec4,
}

impl MaterialExtension for UnderWaterExtention {
  fn fragment_shader() -> ShaderRef {
    MATERIAL_DOWN_PATH.into()
  }
}

// use bevy::prelude::*;
// use bevy::render::render_resource::*;
// use bevy::reflect::TypeUuid;

// #[derive(AsBindGroup, Debug, Clone, TypeUuid)]
// #[uuid = "abcd1234-5678-90ef-1234-567890abcdef"]
// pub struct UnderWaterExtention {
//   #[uniform(0)]
//   pub fog_height: f32,
//   #[uniform(1)]
//   pub fog_color: Vec4,
// }

// impl Material for UnderWaterExtention {
//   fn fragment_shader() -> ShaderRef {
//       "shaders/fog_below_height.wgsl".into()
//   }
// }
