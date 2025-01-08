use avian3d::parry::na::Perspective3;
// use avian3d::parry::na::Perspective3;
// use bevy::prelude::*;
use bevy::asset::{ Asset, Assets };
use bevy::core::Name;
use bevy::core_pipeline::prepass::DepthPrepass;
use bevy::math::Mat4;
use bevy::pbr::MeshMaterial3d;
use bevy::prelude::{ Camera3d, Commands, PerspectiveProjection, Query, ResMut, Transform, With };
use bevy::{ pbr::{ MaterialExtension, Material } };
use bevy::render::{ render_resource::*, camera::{ Camera, RenderTarget } };
use bevy_reflect::Reflect;
// use bevy::render::pass::ClearColor;

const FRAGMENT: &str = "shaders/post-processing/water.wgsl";
const VERTEX: &str = "shaders/post-processing/water.wgsl";

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct PostProcessMaterial {
  #[uniform(110)]
  pub view_matrix: Mat4,
  #[uniform(111)]
  pub projection_matrix: Mat4,
}

impl Material for PostProcessMaterial {
  fn vertex_shader() -> ShaderRef {
    VERTEX.into()
  }

  fn fragment_shader() -> ShaderRef {
    FRAGMENT.into()
  }

  // fn deferred_fragment_shader() -> ShaderRef {
  //   MATERIAL_UP_PATH.into()
  // }
}

fn update_camera_matrices(
  camera_query: Query<(&Transform, &PerspectiveProjection), With<Camera>>,
  mut materials: ResMut<Assets<PostProcessMaterial>>
) {
  if let Ok((camera_transform, perspective_projection)) = camera_query.get_single() {
    let view_matrix = camera_transform.compute_matrix().inverse();

    let projection_matrix: Mat4 = Perspective3::new(
      perspective_projection.aspect_ratio, // Aspect ratio
      perspective_projection.fov, // Field of View
      perspective_projection.near, // Near plane
      perspective_projection.far // Far plane
    )
      .to_homogeneous()
      .into();

    // Pass the matrices to your post-processing material
    for material in materials.iter_mut() {
      material.1.view_matrix = view_matrix;
      material.1.projection_matrix = projection_matrix;
    }
  }
}

fn setup_postprocess(
  mut commands: Commands,
  mut materials: ResMut<Assets<PostProcessMaterial>>,
  camera_query: Query<(&Transform, &PerspectiveProjection), With<Camera>>
) {
  if let Ok((camera_transform, perspective_projection)) = camera_query.get_single() {
    let view_matrix = camera_transform.compute_matrix().inverse();

    let projection_matrix: Mat4 = Perspective3::new(
      perspective_projection.aspect_ratio, // Aspect ratio
      perspective_projection.fov, // Field of View
      perspective_projection.near, // Near plane
      perspective_projection.far // Far plane
    )
      .to_homogeneous()
      .into();

    // Apply the post-processing material
    commands.spawn((
      Name::new("p_player_cam_t"),
      Camera3d::default(),
      // Transform::from_xyz(0.0, 6.0, 0.0), // .looking_at(POS, Vec3::Y),
      Transform::from_xyz(0.0, 1.0, 0.0), // .looking_at(POS, Vec3::Y),
      // PlayerCameraMarker,
      DepthPrepass,
      // NormalPrepass,
    ));
    // .insert(PostProcessMaterial {
    //   view_matrix,
    //   projection_matrix,
    // });
  }
}
