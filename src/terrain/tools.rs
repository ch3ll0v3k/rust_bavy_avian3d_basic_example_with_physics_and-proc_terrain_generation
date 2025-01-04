use std::collections::HashMap;

use bevy::{
  asset::{ AssetServer, Handle },
  color::{ Color, LinearRgba },
  image::Image,
  math::{ Affine2, Vec2 },
  pbr::{ OpaqueRendererMethod, StandardMaterial },
  prelude::{ AlphaMode, Cuboid, Entity, Mesh, Res, ResMut, Resource },
  utils::default,
};
use wgpu::Face;

use crate::{ asset_loader::image_cache::{ cache_load_image, ImageCache }, sys_paths::image::pbr };

use super::terrain_constants::*;

pub struct IInnerMap {
  pub entity: Entity,
  pub lod: i16,
}

#[derive(Resource, Default)]
pub struct InnerMapper {
  pub hash_map: HashMap<(i16, i16), IInnerMap>,
}

impl InnerMapper {
  pub fn new() -> Self {
    Self {
      hash_map: HashMap::new(),
    }
  }
}

// prettier-ignore
pub fn get_terrain_bpr(
  asset_server: &Res<AssetServer>,
  image_hashmap: &mut ResMut<ImageCache>,
) -> StandardMaterial{

  let uv_transform: Vec2 = Vec2::new(
    TERRAIN_STATIC_ON_MATERIAL_UV_SCALE, 
    TERRAIN_STATIC_ON_MATERIAL_UV_SCALE
  );

  let terrain_pbr_diff_handle: Handle<Image> = cache_load_image(
    image_hashmap,
    asset_server, 
    pbr::aerial_grass_rock::AerialGrassRock::DiffLight.as_str(),
    true
  );

  let terrain_pbr_norm_handle: Handle<Image> = cache_load_image(
    image_hashmap,
    asset_server, 
    pbr::aerial_grass_rock::AerialGrassRock::NorGl.as_str(),
    true
  );

  let terrain_pbr_rough_handle: Handle<Image> = cache_load_image(
    image_hashmap,
    asset_server, 
    pbr::aerial_grass_rock::AerialGrassRock::Rough.as_str(),
    true
  );

  let terrain_pbr_ao_handle: Handle<Image> = cache_load_image(
    image_hashmap,
    asset_server, 
    pbr::aerial_grass_rock::AerialGrassRock::Ao.as_str(),
    true
  );

  let mut terrain_material: StandardMaterial = StandardMaterial {
    base_color_texture: Some(terrain_pbr_diff_handle.clone()),
    normal_map_texture: Some(terrain_pbr_norm_handle.clone()),
    metallic_roughness_texture: Some(terrain_pbr_rough_handle.clone()),
    occlusion_texture: Some(terrain_pbr_ao_handle.clone()),
    // emissive_texture,
    uv_transform: Affine2::from_scale(uv_transform),
    // alpha_mode: AlphaMode::Blend,
    unlit: false,
    emissive: LinearRgba::BLACK,
    // emissive_exposure_weight: 1.0,
    perceptual_roughness: 0.85,
    // metallic: 0.0,
    reflectance: 0.05,
    // ior: 1.47,
    ..default()
  };

  // terrain_material.uv_transform = Affine2::from_scale(Vec2::new(
  //   TERRAIN_STATIC_ON_MATERIAL_UV_SCALE, 
  //   TERRAIN_STATIC_ON_MATERIAL_UV_SCALE
  // ));

  terrain_material


}

pub fn get_water_pbr_and_mesh() -> (StandardMaterial, Mesh) {
  let mut water: Mesh = Mesh::from(Cuboid::new(TERRAIN_CHUNK_X, 0.1, TERRAIN_CHUNK_X))
    .with_generated_tangents()
    .unwrap();
  water.compute_normals();

  let water_material: StandardMaterial = StandardMaterial {
    unlit: !false,
    double_sided: true,
    cull_mode: Some(Face::Front),
    base_color: Color::srgba_u8(70, 70, 180, 17),
    opaque_render_method: OpaqueRendererMethod::Auto,
    alpha_mode: AlphaMode::Blend,
    ..default()
  };

  return (water_material, water);
}

// testing ....

// #[derive(Resource)]
// pub struct IMapTestShift {
//   pub x: f64,
//   pub z: f64,
// }
