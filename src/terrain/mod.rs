use core::hash;
use std::{ f32::consts::PI, time::Duration };

// prettier-ignore
use avian3d::prelude::{ AngularVelocity, Collider, CollisionMargin, PhysicsSet, RigidBody, Sensor };

// prettier-ignore
use bevy::{
  app::{ App, ScheduleRunnerPlugin, Startup, Update },
  asset::{ AssetServer, Assets, Handle },
  audio::{ AudioPlayer, AudioPlugin, AudioSource, PlaybackMode, PlaybackSettings, Volume },
  color::{ palettes::{ css::*, tailwind::* }, Color },
  core_pipeline::{
    core_3d::graph::{ Core3d, Node3d },
    fullscreen_vertex_shader::fullscreen_shader_vertex_state,
  },
  ecs::query::{ QueryItem, QuerySingleError },
  gizmos::AppGizmoBuilder,
  image::{
    ImageAddressMode,
    ImageFilterMode,
    ImageLoaderSettings,
    ImageSampler,
    ImageSamplerDescriptor,
  },
  math::{ Affine2, IVec2, Vec2, Vec3 },
  pbr::{
    wireframe::Wireframe,
    CascadeShadowConfigBuilder,
    ExtendedMaterial,
    NotShadowCaster,
    NotShadowReceiver,
    OpaqueRendererMethod,
    StandardMaterial,
  },
  prelude::*,
  render::{
    extract_component::{
      ComponentUniforms,
      DynamicUniformIndex,
      ExtractComponent,
      ExtractComponentPlugin,
      UniformComponentPlugin,
    },
    mesh::VertexAttributeValues,
    render_graph::{
      NodeRunError,
      RenderGraphApp,
      RenderGraphContext,
      RenderLabel,
      ViewNode,
      ViewNodeRunner,
    },
    render_resource::binding_types::{ sampler, texture_2d, uniform_buffer },
    renderer::{ RenderContext, RenderDevice },
    view::ViewTarget,
    RenderApp,
  },
  time::{ common_conditions::on_timer, Fixed, Time },
  utils::{ default, hashbrown::hash_map },
  window::WindowMode::*,
};

mod terrain_lod_map;
mod tools;
mod terrain_constants;

use rand::Rng;
use terrain_lod_map::TERRAIN_LOD_MAP_SIZE;
use wgpu::Face;
use noise::{ BasicMulti, NoiseFn, Perlin };
use std::collections::HashMap;

use terrain_constants::*;
use tools::*;

// prettier-ignore
use crate::{
  asset_loader::image_cache::{ cache_load_image, ImageCache },
  dbgln,
  debug::get_defaul_physic_debug_params,
  materials::{ post_processing::water, water::WaterExtension },
  player::PlayerMarker,
  sys_paths::{ self, image::EImageTreeBase },
  terrain::terrain_lod_map::get_lod,
  AnyObject,
  PhysicsStaticObject,
  PhysicsStaticObjectTerrain,
  COLLISION_MARGIN,
  // materials::water::{ UnderWaterExtention, WaterExtension },
};

// prettier-ignore
use sys_paths::{ audio::EAudio, image::{ EImageWaterBase, EImageTerrainBase, pbr } };

#[derive(Component, Debug, PartialEq, Eq)]
pub struct MTerrainMarker;

#[derive(Component, Debug, PartialEq, Eq)]
pub struct MTerrainPlugin;

// prettier-ignore
impl Plugin for MTerrainPlugin {
  fn build(&self, app: &mut App) {
    app.insert_resource(InnerMapper::new());
    // .insert_resource(IMapTestShift{ x: 0.0, z: 0.0 });

    // app
    //   .add_plugins((
    //     MaterialPlugin::<ExtendedMaterial<StandardMaterial, WaterExtension>>::default()
    //   ));

    app
      .add_systems(Startup, startup)
      // .add_systems(Update, (
      //   update_terrain_on_player_position,
      //   // modify_mesh_at_runtime,
      // ))
      .add_systems(
        PostUpdate,
        (update_terrain_on_player_position, update_far_tree_look_at).run_if(
          on_timer(Duration::from_millis(100))
        )
      );
    // .add_systems(PostUpdate, (
    //   draw_cursor
    // ).run_if(
    //   on_timer(Duration::from_millis(100))
    // ));
  }
}

// prettier-ignore
fn startup(
  mut res_mut_texture_cache: Option<ResMut</*res_mut_texture_cache::*/ ImageCache>>,
  mut inner_mapper_mut: Option<ResMut<InnerMapper>>,
  asset_server: Res<AssetServer>,
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>
) {
  // return;

  // // XXX
  // let mut inner_map: &mut ResMut<'_, InnerMapper> = inner_mapper_mut.as_mut().unwrap();
  // let image_hashmap: &mut ResMut<ImageCache> = res_mut_texture_cache.as_mut().unwrap();
  // let (water_material, water ) = get_water_pbr_and_mesh();
  // let water_material_handle: Handle<StandardMaterial> = materials.add(water_material);

  // let terrain_material: StandardMaterial = get_terrain_bpr(&asset_server, image_hashmap);
  // let terrain_material_handle: Handle<StandardMaterial> = materials.add(terrain_material);

  // let lod: [[i16; TERRAIN_LOD_MAP_SIZE]; TERRAIN_LOD_MAP_SIZE] = get_lod();

  // for z in -TERRAIN_SEGMENTS_TO_GEN..=TERRAIN_SEGMENTS_TO_GEN {
  //   for x in -TERRAIN_SEGMENTS_TO_GEN..=TERRAIN_SEGMENTS_TO_GEN {

  //     let on_z = ((TERRAIN_LOD_MAP_SIZE as i32 - 7) + z) as usize;
  //     let on_x = ((TERRAIN_LOD_MAP_SIZE as i32 - 7) + x) as usize;

  //     let dyn_scale = lod[ on_z ][ on_x ] as i16;

  //     if dyn_scale <= 0 { continue; }
  //     let terrain: Mesh = generate_chunk(x as f64, z as f64, dyn_scale);

  //     let terrain_id: Entity = commands.spawn((
  //       RigidBody::Static,
  //       CollisionMargin(COLLISION_MARGIN),
  //       Collider::trimesh_from_mesh(&terrain).unwrap(),
  //       Mesh3d(meshes.add(terrain)),
  //       MeshMaterial3d(terrain_material_handle.clone()),
  //       // MeshMaterial3d(materials.add(Color::srgb_u8(255, 255, 255))),
  //       MTerrainMarker,
  //       PhysicsStaticObject,
  //       PhysicsStaticObjectTerrain,
  //       get_defaul_physic_debug_params(),
  //       AnyObject,
  //       Name::new("terrain_t"),
  //     )).id();

  //     let mut spawn: bool = false;

  //     if let Some(res) = inner_map.hash_map.get(&(z as i16, x as i16)) {
  //       dbgln!("inner_map.hash_map.get(&({z}, {x})) => lod (load): {}", res.lod);

  //       if( res.lod != dyn_scale ){
  //         dbgln!("inner_map.hash_map.get(&({z}, {x})) => lod (dyn) => commands.entity({}).despawn: {}", res.entity, dyn_scale);
  //         spawn = true;
  //         commands.entity(res.entity).despawn();
  //       }

  //     }else{
  //       dbgln!("inner_map.hash_map.insert(&({z}, {x})) => lod (dyn): {dyn_scale}");
  //       spawn = true;
  //     }

  //     if( spawn ){
  //       let capacity: Option<IInnerMap> = inner_map.hash_map.insert(
  //         (z as i16, x as i16),
  //         IInnerMap{
  //             // entity: terrain_id,
  //             entity: Entity::from( terrain_id ),
  //             lod: dyn_scale
  //           }
  //       );
  //     }

  //     // if let Some(res_mut) = &mut inner_mapper_mut {
  //     //   // dbgln!("capacity: {:?}", res_mut.hash_map.capacity());
  //     //   if let Some(res) = res_mut.hash_map.get(&(z as i16, x as i16)) {
  //     //     // dbgln!("res_mut.hash_map.get(&({z}, {x})) => lod: {}", res.lod);
  //     //   }else{
  //     //     // dbgln!("res_mut.hash_map.insert(&({z}, {x})) => lod: {dyn_scale}");
  //     //     let capacity = res_mut.hash_map.insert(
  //     //       (z as i16, x as i16),
  //     //       IInnerMap{
  //     //           // entity: terrain_id,
  //     //           entity: Entity::from( terrain_id ),
  //     //           // entity: Entity::from_raw(42s31231231),
  //     //           lod: dyn_scale
  //     //         }
  //     //     );
  //     //   }
  //     // }

  //     let walter_f: i32 = 0;

  //     if z >= -walter_f && z <= walter_f && x >= -walter_f && x <= walter_f {

  //       commands.spawn((
  //         // RigidBody::Static,
  //         // Collider::trimesh_from_mesh(&water).unwrap(),
  //         // Sensor,
  //         Transform::from_xyz(
  //           (x * TERRAIN_CHUNK_X as i32) as f32,
  //            -3.0, // -13
  //           (z * TERRAIN_CHUNK_X as i32) as f32
  //           // .looking_at(Vec3::ZERO, Vec3::ZERO)
  //         ),
  //         Mesh3d(meshes.add(water.clone())),
  //         // MeshMaterial3d(materials.add(Color::srgba_u8(128, 197, 222,17))),
  //         // MeshMaterial3d(water_material_handle.clone()),
  //         MeshMaterial3d(water_material_handle.clone()),
  //         // DebugRender::default().with_collider_color(Color::srgb(1.0, 0.0, 1.0)),
  //         AnyObject,
  //         Name::new("water_t"),
  //       ));
  //     }

  //   }
  // }

}

fn round_upto(num: f64, upto: i8) -> f64 {
  let pow = (10.0 as f64).powi(upto as i32);
  (num * pow).round() / pow
}

// fn draw_cursor(
//   // camera_query: Single<(&Camera, &GlobalTransform)>,
//   // ground: Single<&GlobalTransform, With<MTerrainMarker>>,
//   camera_query: Query<(&Camera, &GlobalTransform)>,
//   ground: Query<&GlobalTransform, With<MTerrainMarker>>,
//   windows: Single<&Window>,
//   mut gizmos: Gizmos
// ) {
//   let (camera, camera_transform) = camera_query.single();

//   let Some(cursor_position) = windows.cursor_position() else {
//     return;
//   };

//   // Calculate a ray pointing from the camera into the world based on the cursor's position.
//   let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
//     return;
//   };

//   for g_trans in ground.iter() {
//     // Calculate if and where the ray is hitting the ground plane.
//     dbgln!("g_trans: {g_trans:?}");
//     // Calculate if and where the ray is hitting the ground plane.
//     let Some(distance) = ray.intersect_plane(
//       g_trans.translation(),
//       InfinitePlane3d::new(g_trans.up())
//     ) else {
//       dbgln!("Nooope ...");
//       return;
//     };
//     dbgln!("distance: {distance:?}");

//     let point = ray.get_point(distance);

//     // Draw a circle just above the ground plane at that position.
//     gizmos.circle(
//       Isometry3d::new(
//         point + g_trans.up() * 0.01,
//         Quat::from_rotation_arc(Vec3::Z, g_trans.up().as_vec3())
//       ),
//       0.2,
//       Color::WHITE
//     );
//   }
// }

#[derive(Component, Debug)]
pub struct MTreeMarker;

// prettier-ignore
fn update_far_tree_look_at(
  q_player: Query<&Transform, (With<PlayerMarker>, Without<MTreeMarker>)>,
  mut q_trees: Query<&mut Transform, (With<MTreeMarker>, Without<PlayerMarker>)>
) {
  let trans = q_player.single().translation;
  for mut q_tree in q_trees.iter_mut() {
    let x = trans.x;
    let y = q_tree.translation.y;
    let z = trans.z;
    let vec3 = Vec3::new(x, y, z);

    q_tree.look_at(vec3, Vec3::ZERO);
    q_tree.rotate_local_z(PI / 1.0);
  }
}

// prettier-ignore
fn update_terrain_on_player_position(
  // mut water_materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, WaterExtension>>>,
  asset_server: Res<AssetServer>,
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  mut inner_mapper_mut: Option<ResMut<InnerMapper>>,
  // mut inner_mapper_read: Res<InnerMapper>,
  mut res_mut_texture_cache: Option<ResMut</*res_mut_texture_cache::*/ ImageCache>>,
  mut q_player: Query<&mut Transform, (With<PlayerMarker>, Without<MTerrainMarker>)>
  // q_name: Query<&Name>,
  // mut q_terrain: Query<
  //   (Entity, &mut RigidBody, &mut Transform),
  //   (With<MTerrainMarker>, Without<PlayerMarker>)
  // >,
) {
  // dbgln!(" ---------- ---------- ---------- ---------- ---------- ");
  let mut inner_map_mut = &mut inner_mapper_mut.as_mut().unwrap();
  // let mut inner_map_read = &inner_mapper_read;

  let image_hashmap: &mut ResMut<ImageCache> = res_mut_texture_cache.as_mut().unwrap();

  let o_player: Mut<'_, Transform> = q_player.single_mut();
  let pos: Vec3 = o_player.translation;

  let add_x: f32 = if pos.x < 0.0 { -TERRAIN_CHUNK_X_HALF } else { TERRAIN_CHUNK_X_HALF };
  let add_z: f32 = if pos.z < 0.0 { -TERRAIN_CHUNK_X_HALF } else { TERRAIN_CHUNK_X_HALF };
  let g_x: i32 = ((pos.x + add_x) / TERRAIN_CHUNK_X) as i32;
  let g_z: i32 = ((pos.z + add_z) / TERRAIN_CHUNK_X) as i32;
  let lod: [[i16; TERRAIN_LOD_MAP_SIZE]; TERRAIN_LOD_MAP_SIZE] = get_lod();

  const SIZE_SIZE_T: i32 = 7; // ((TERRAIN_LOD_MAP_SIZE - 1)/2) as i32;
  // dbgln!("SIZE_SIZE_T: {SIZE_SIZE_T}");

  for l_z in -SIZE_SIZE_T..=SIZE_SIZE_T {
    for l_x in -SIZE_SIZE_T..=SIZE_SIZE_T {
      let lod_on_z: usize = ((TERRAIN_LOD_MAP_SIZE as i32) - 8 + l_z) as usize;
      let lod_on_x: usize = ((TERRAIN_LOD_MAP_SIZE as i32) - 8 + l_x) as usize;
      // dbgln!("lod_on_z: {lod_on_z}, lod_on_x: {lod_on_x}");
      let dyn_scale: i16 = lod[lod_on_z][lod_on_x] as i16;
      let abs_x: i32 = g_x + l_x;
      let abs_z: i32 = g_z + l_z;

      let mut spawn: bool = false;
      let mut remove_from_map: bool = false;

      let map_item_item = inner_map_mut.hash_map.get(&(abs_z as i16, abs_x as i16));

      if let Some(res) = map_item_item {
        // dbgln!("  => found (&({abs_z}, {abs_x})) => lod ({}), dyn_scale: {}", res.lod, dyn_scale);

        if 0 == dyn_scale {
          // dbgln!("  (g: {g_z}/{g_x}) => (l: {l_z}/{l_x}) =>  (abs: {abs_z}/{abs_x}) => dyn_scale: {dyn_scale}");
          // dbgln!("  => [0] commands.entity(res.entity).despawn(&({abs_z}, {abs_x})) => despawn");
          remove_from_map = true;
          commands.entity(res.entity).despawn();
        } else if res.lod != dyn_scale {
          // dbgln!("  (g: {g_z}/{g_x}) => (l: {l_z}/{l_x}) =>  (abs: {abs_z}/{abs_x}) => dyn_scale: {dyn_scale}");
          // dbgln!("  => [1] commands.entity(res.entity).despawn(&({abs_z}, {abs_x})) => despawn");
          // inner_map_mut.hash_map.remove(&(abs_z as i16, abs_x as i16));
          remove_from_map = true;
          commands.entity(res.entity).despawn();
          spawn = true;
        } else {
          // dbgln!("  => [2] entity @ (&({abs_z}, {abs_x})) => same LOD: (dyn_scale: {dyn_scale}, res.lod: {})",res.lod);
        }
      } else {
        // dbgln!("inner_map_mut.hash_map.insert(&({abs_z}, {abs_x})) => lod (dyn): {dyn_scale}");
        if dyn_scale > 0 {
          spawn = true;
        }
      }

      if remove_from_map {
        inner_map_mut.hash_map.remove(&(abs_z as i16, abs_x as i16));
      }

      if spawn {
        // dbgln!("  => SPAWN @(&({abs_z}, {abs_x})) => lod (dyn): {dyn_scale}");

        let (terrain, tree_vec) = generate_chunk(abs_x as f64, abs_z as f64, dyn_scale);
        let terrain_material: StandardMaterial = get_terrain_bpr(
          &asset_server,
          image_hashmap,
          dyn_scale
        );
        let terrain_material_handle: Handle<StandardMaterial> = materials.add(terrain_material);

        let terrain_id: Entity = commands
          .spawn((
            RigidBody::Static,
            CollisionMargin(COLLISION_MARGIN),
            Collider::trimesh_from_mesh(&terrain).unwrap(),
            Mesh3d(meshes.add(terrain)),
            MeshMaterial3d(terrain_material_handle.clone()),
            // MeshMaterial3d(materials.add(Color::srgb_u8(255, 255, 255))),
            MTerrainMarker,
            PhysicsStaticObject,
            PhysicsStaticObjectTerrain,
            get_defaul_physic_debug_params(),
            AnyObject,
            Name::new("terrain_t"),
            // Wireframe::default(),
          ))
          .with_children(|parent| {})
          .id();

        let ins: Option<IInnerMap> = inner_map_mut.hash_map.insert(
          (abs_z as i16, abs_x as i16),
          IInnerMap {
            // entity: terrain_id,
            entity: Entity::from(terrain_id),
            lod: dyn_scale,
          }
        );

        let t_w = 600.0 * 3.0;
        let t_h = 700.0 * 3.0;

        // let mut rng = rand::thread_rng();
        // let random_seed_y: f32 = rng.gen();
        // let random_seed_z: f32 = rng.gen();

        for vec in tree_vec {
          let x = vec[0];
          let y = vec[1];
          let z = vec[2];

          let tree_tex: Handle<Image> = asset_server.load(EImageTreeBase::Two.as_str());
          let tree_mat: StandardMaterial = StandardMaterial {
            base_color_texture: Some(tree_tex),
            double_sided: false,
            cull_mode: Some(Face::Front),
            // cull_mode: Some(Face::Back),
            unlit: !false,
            // base_color: Color::srgba_u8(255, 40, 40, 20),
            // base_color: Color::srgba_u8(255, 255, 255, 255),
            opaque_render_method: OpaqueRendererMethod::Auto,
            alpha_mode: AlphaMode::Blend,
            // alpha_mode: AlphaMode::Mask(0.5),
            ..default()
          };
          let tree_handle = materials.add(tree_mat);
          let mut tree_mesh: Mesh;

          if true {
            tree_mesh = Mesh::from(Cuboid::new(t_w, t_h, 0.01));
            // .with_generated_tangents()
            // .unwrap();
          } else {
            tree_mesh = Mesh::from(Plane3d::default().mesh().size(t_w, t_h).subdivisions(0));
          }

          commands.spawn((
            Name::new("tree_t"),
            Mesh3d(meshes.add(tree_mesh.clone())),
            MeshMaterial3d(tree_handle.clone()),
            Transform {
              translation: Vec3::new(x, y + (-50.0 + t_h / 2.0), z),
              rotation: Quat::from_axis_angle(Vec3::Y, 3.1415),
              ..default()
            },
            MTreeMarker,
            NotShadowCaster,
            NotShadowReceiver,
            AnyObject,
          ));
          // .with_children(|parent|{
          //   parent
          //     .spawn((
          //       Mesh3d(meshes.add(tree_mesh.clone())),
          //       MeshMaterial3d(tree_handle.clone()),
          //       Transform {
          //         translation: Vec3::new(0.0, 0.0, 0.0),
          //         rotation: Quat::from_axis_angle(Vec3::Y, 3.1415 / 2.0),
          //         ..default()
          //       },
          //       NotShadowCaster,
          //       NotShadowReceiver,
          //       AnyObject,
          //     ));
          // });
        }

        continue;

        // let water_diff_texture: Handle<Image> = cache_load_image(
        //   image_hashmap,
        //   &asset_server,
        //   EImageWaterBase::Walet1Base.as_str(),
        //   true
        // );

        // let water_normal_map_texture: Handle<Image> = cache_load_image(
        //   image_hashmap,
        //   &asset_server,
        //   EImageWaterBase::Walet1Normal.as_str(),
        //   true
        // );

        // shaders with DepthPrepass => very slow
        // {
        //   let water_material = StandardMaterial{
        //     // base_color_texture: Some(water_diff_texture.clone()),
        //     // normal_map_texture: Some(water_normal_map_texture.clone()),
        //     // uv_transform: Affine2::from_scale(Vec2::new(4.0, 4.0)),
        //     // base_color: BLUE_400.into(),
        //     metallic: 0.3,
        //     base_color: Color::srgba_u8(128, 197, 222, 30),
        //     perceptual_roughness: 0.8,
        //     alpha_mode: AlphaMode::Blend,
        //     opaque_render_method: OpaqueRendererMethod::Auto,
        //     ..default()
        //   };
        //   let water_material_handle: Handle<ExtendedMaterial<StandardMaterial, WaterExtension>> = water_materials.add(ExtendedMaterial {
        //     base: water_material,
        //     extension: WaterExtension {
        //       quantize_steps: 30
        //     },
        //   });
        // }

        // if z >= -walter_f && z <= walter_f && x >= -walter_f && x <= walter_f {}
        if g_x == 0 && g_z == 0 {
          let walter_f: i32 = 0;
          // basic material
          let (water_material, water) = get_water_pbr_and_mesh();
          let water_material_handle: Handle<StandardMaterial> = materials.add(water_material);

          commands.spawn((
            // RigidBody::Static,
            // Collider::trimesh_from_mesh(&water).unwrap(),
            // Sensor,
            Transform::from_xyz(
              (g_x * (TERRAIN_CHUNK_X as i32)) as f32, // - 1000.0,
              // -3.0 + -0.1, // -13
              -3.0 + -0.1, // -13
              (g_z * (TERRAIN_CHUNK_X as i32)) as f32 // - 1000.0
              // .looking_at(Vec3::ZERO, Vec3::ZERO)
            ),
            Mesh3d(meshes.add(water.clone())),
            // MeshMaterial3d(materials.add(Color::srgba_u8(128, 197, 222,17))),
            // MeshMaterial3d(water_material_handle.clone()),
            MeshMaterial3d(water_material_handle.clone()),
            // DebugRender::default().with_collider_color(Color::srgb(1.0, 0.0, 1.0)),
            AnyObject,
            Name::new("water_t"),
          ));
        }
      }
    }
  }
}

// prettier-ignore
// fn modify_mesh_at_runtime(
//   mut res_mut_map_shift: Option<ResMut<IMapTestShift>>,
//   mut meshes: ResMut<Assets<Mesh>>,
//   // query: Query<&Handle<Mesh>, With<MTerrainMarker>>,
//   // mut qu: Query<(Entity, &mut Handle<Mesh3d>), With<MTerrainMarker>>,
//   query: Query<&Mesh3d, With<MTerrainMarker>>,
// ) {

//   let map: &mut ResMut<'_, IMapTestShift> = res_mut_map_shift.as_mut().unwrap();

//   // map.x = 1.0;
//   // map.z = 1.0;

//   map.x = round_upto(map.x - 0.00000001);
//   // map.z = round_upto(map.z + 0.00000001);
//   dbgln!("map shift: (x: {} z: {})", map.x, map.z);

//   // dbgln!(" ----------------------------------------------- ");
//   // meshes.iter_mut().for_each(|(handle, mesh)| {
//   //   dbgln!("mesh: {handle} => {:?}", mesh);
//   // });

//   let noise: BasicMulti<Perlin> = BasicMulti::<Perlin>::default();
//   let final_subdivisions: u32 = calculate_final_subdivisions(4);
//   // dbgln!("chunk-size: {TERRAIN_CHUNK_X} => (base-subdiv: {TERRAIN_CHUNK_SUBDIVISIONS}, dyn_scale: {dyn_scale}) => final-subdiv: {final_subdivisions}");

//   let mut x = map.x;
//   let mut z = map.z;
//   dbgln!("map shift: (x: {} z: {})", x, z);
//   // let x = 0.0;
//   // let z = 0.0;

//   let dyn_scale: i32 = -1;

//   let mut min: f32 = f32::MAX;
//   let mut max: f32 = -f32::MAX;

//   for mesh_handle in query.iter() {
//     if let Some(terrain) = meshes.get_mut(mesh_handle) {
//       // Modify the mesh data here, e.g., updating vertices or normals

//       if let Some(VertexAttributeValues::Float32x3(positions)) = terrain.attribute_mut(Mesh::ATTRIBUTE_POSITION) {
//         // main terrain topology
//         for pos in positions.iter_mut() {
//           let xi: f32 = noise.get([
//             (((pos[0] as f64) + (TERRAIN_CHUNK_X as f64) * x) as f64) / TERRAIN_CHUNK_SCALLER,
//             (((pos[2] as f64) + (TERRAIN_CHUNK_X as f64) * z) as f64) / TERRAIN_CHUNK_SCALLER,
//             0.0 as f64,
//           ]) as f32;
//           pos[0] += (TERRAIN_CHUNK_X * (x as f32)) as f32; // + ((x / 1.0) as f32);
//           pos[1] = xi * TERRAIN_HEIGHT * 1.0;
//           // pos[1] = 0.0;
//           pos[2] += (TERRAIN_CHUNK_X * (z as f32)) as f32; // + ((z / 1.0) as f32);
//           // if USE_SEGMENT_SEPARATOR {
//           //   pos[0] += (x / 1.0) as f32;
//           //   pos[2] += (z / 1.0) as f32;
//           // }
//         }

//         for pos in positions.iter_mut() {
//           pos[1] *= 1.50;
//         }

//         // waler down
//         // for pos in positions.iter_mut() {
//         //   pos[1] -= 10000.0; // def: 1.0
//         // }

//         for pos in positions.iter_mut() {
//           pos[1] += 20.0; // def: 1.0
//         }

//         let sub = 7.0; // 15.0; // -10.0;

//         let colors: Vec<[f32; 4]> = positions
//           .iter()
//           .map(|[_, g, _]| {
//             let g: f32 = ((*g-sub) + TERRAIN_HEIGHT) / (TERRAIN_HEIGHT * 2.0); //  * 2.0 + 2.0; // * 26.0;
//             min = if min > g { g } else { min };
//             max = if max < g { g } else { max };
//             return terrain_cal_color_on_g(g);
//           })
//           .collect();
//         terrain.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
//         terrain.compute_normals();

//         if TERRAIN_USE_LOWER_Y_ON_FAR_DISTANCE {
//           if let Some(VertexAttributeValues::Float32x3(positions)) = terrain.attribute_mut(Mesh::ATTRIBUTE_POSITION ){
//             for pos in positions.iter_mut() {
//               pos[1] -= (((dyn_scale.abs() as f32) - 4.0) * 1.0) / 2.0;
//             }
//           }
//         }

//       }
//     }
//   }
// }
