#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unreachable_code)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_braces)]
#![allow(unused_parens)]

#[macro_use]
mod debug_utils;

use avian3d::debug_render::{ PhysicsDebugPlugin, DebugRender };
use avian3d::prelude::*;
use avian3d::PhysicsPlugins;

use bevy::prelude::*;
use asset_loader::audio_cache::{ cache_load_audio, AudioCache };

use bevy::app::{ ScheduleRunnerPlugin, App, Startup, Update };
use bevy::asset::{ AssetServer, Assets, Handle };
use bevy::audio::AudioPlugin;
use bevy::audio::{ AudioPlayer, AudioSource, PlaybackSettings, PlaybackMode, Volume };
use bevy::color::{ Color, palettes::css::*, palettes::tailwind::* };

use bevy::gizmos::AppGizmoBuilder;
use bevy::image::{
  ImageAddressMode,
  ImageFilterMode,
  ImageLoaderSettings,
  ImageSampler,
  ImageSamplerDescriptor,
};

use bevy::math::{ IVec2, Vec2, Vec3 };
use bevy::pbr::StandardMaterial;
use bevy::time::{ Time, Fixed, common_conditions::on_timer };
use bevy::utils::default;
use bevy_window::{
  WindowResolution,
  WindowLevel,
  PresentMode,
  Window,
  WindowPlugin,
  WindowPosition,
};

use bevy::pbr::{ CascadeShadowConfigBuilder, ExtendedMaterial, OpaqueRendererMethod };

use bevy::{
  core_pipeline::{
    core_3d::graph::{ Core3d, Node3d },
    fullscreen_vertex_shader::fullscreen_shader_vertex_state,
  },
  ecs::query::QueryItem,
  prelude::*,
  render::{
    extract_component::{
      ComponentUniforms,
      DynamicUniformIndex,
      ExtractComponent,
      ExtractComponentPlugin,
      UniformComponentPlugin,
    },
    render_graph::{
      NodeRunError,
      RenderGraphApp,
      RenderGraphContext,
      RenderLabel,
      ViewNode,
      ViewNodeRunner,
    },
    render_resource::{ binding_types::{ sampler, texture_2d, uniform_buffer }, * },
    renderer::{ RenderContext, RenderDevice },
    view::ViewTarget,
    RenderApp,
  },
};

use instant::Instant;
use noise::{ BasicMulti, NoiseFn, Perlin };

use bevy::ecs::query::QuerySingleError;
use bevy::render::mesh::VertexAttributeValues;
use bevy::window::WindowMode::*;
use std::{ collections::HashMap, time::Duration };

mod camera;
mod cubes;
mod debug;
mod lights;
mod markers;
mod constants;
mod terrain;
mod sky;
mod entities;
mod state;
mod sys_paths;
mod asset_loader;
mod m_lib;
mod materials;

use camera::CameraMarker;
use debug::get_defaul_physic_debug_params;
use lights::MPointLightMarker;
use markers::{ m_avian::*, m_bevy::* };
use constants::{ viewport_settings::*, physics_world::* };
use terrain::MTerrainMarker;
use sys_paths::audio::EAudio;
use camera::CameraParentMarker;
use m_lib::physics;

const WINDOW_POSITIONS_DEV_SIDE_33_PERCENT: Vec2 = Vec2::new(800.0, 1100.0);
const WINDOW_POSITIONS_DEV_SIDE_50_PERCENT: Vec2 = Vec2::new(950.0, 1100.0);
const USE_WIN_SIZE: Vec2 = WINDOW_POSITIONS_DEV_SIDE_50_PERCENT;

const SHADER_ASSET_PATH: &str = "shaders/post-processing/example.wgsl";

struct PostProcessPlugin;

impl Plugin for PostProcessPlugin {
  fn build(&self, app: &mut App) {
    app.add_plugins((
      // The settings will be a component that lives in the main world but will
      // be extracted to the render world every frame.
      // This makes it possible to control the effect from the main world.
      // This plugin will take care of extracting it automatically.
      // It's important to derive [`ExtractComponent`] on [`PostProcessingSettings`]
      // for this plugin to work correctly.
      ExtractComponentPlugin::<PostProcessSettings>::default(),
      // The settings will also be the data used in the shader.
      // This plugin will prepare the component for the GPU by creating a uniform buffer
      // and writing the data to that buffer every frame.
      UniformComponentPlugin::<PostProcessSettings>::default(),
    ));

    // We need to get the render app from the main app
    let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
      return;
    };

    render_app
      // Bevy's renderer uses a render graph which is a collection of nodes in a directed acyclic graph.
      // It currently runs on each view/camera and executes each node in the specified order.
      // It will make sure that any node that needs a dependency from another node
      // only runs when that dependency is done.
      //
      // Each node can execute arbitrary work, but it generally runs at least one render pass.
      // A node only has access to the render world, so if you need data from the main world
      // you need to extract it manually or with the plugin like above.
      // Add a [`Node`] to the [`RenderGraph`]
      // The Node needs to impl FromWorld
      //
      // The [`ViewNodeRunner`] is a special [`Node`] that will automatically run the node for each view
      // matching the [`ViewQuery`]
      .add_render_graph_node::<ViewNodeRunner<PostProcessNode>>(
        // Specify the label of the graph, in this case we want the graph for 3d
        Core3d,
        // It also needs the label of the node
        PostProcessLabel
      )
      .add_render_graph_edges(
        Core3d,
        // Specify the node ordering.
        // This will automatically create all required node edges to enforce the given ordering.
        (Node3d::Tonemapping, PostProcessLabel, Node3d::EndMainPassPostProcessing)
      );
  }

  fn finish(&self, app: &mut App) {
    // We need to get the render app from the main app
    let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
      return;
    };

    render_app
      // Initialize the pipeline
      .init_resource::<PostProcessPipeline>();
  }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
struct PostProcessLabel;

// The post process node used for the render graph
#[derive(Default)]
struct PostProcessNode;

// The ViewNode trait is required by the ViewNodeRunner
impl ViewNode for PostProcessNode {
  // The node needs a query to gather data from the ECS in order to do its rendering,
  // but it's not a normal system so we need to define it manually.
  //
  // This query will only run on the view entity
  type ViewQuery = (
    &'static ViewTarget,
    // This makes sure the node only runs on cameras with the PostProcessSettings component
    &'static PostProcessSettings,
    // As there could be multiple post processing components sent to the GPU (one per camera),
    // we need to get the index of the one that is associated with the current view.
    &'static DynamicUniformIndex<PostProcessSettings>,
  );

  // Runs the node logic
  // This is where you encode draw commands.
  //
  // This will run on every view on which the graph is running.
  // If you don't want your effect to run on every camera,
  // you'll need to make sure you have a marker component as part of [`ViewQuery`]
  // to identify which camera(s) should run the effect.
  fn run(
    &self,
    _graph: &mut RenderGraphContext,
    render_context: &mut RenderContext,
    (view_target, _post_process_settings, settings_index): QueryItem<Self::ViewQuery>,
    world: &World
  ) -> Result<(), NodeRunError> {
    // Get the pipeline resource that contains the global data we need
    // to create the render pipeline
    let post_process_pipeline = world.resource::<PostProcessPipeline>();

    // The pipeline cache is a cache of all previously created pipelines.
    // It is required to avoid creating a new pipeline each frame,
    // which is expensive due to shader compilation.
    let pipeline_cache = world.resource::<PipelineCache>();

    // Get the pipeline from the cache
    let Some(pipeline) = pipeline_cache.get_render_pipeline(
      post_process_pipeline.pipeline_id
    ) else {
      return Ok(());
    };

    // Get the settings uniform binding
    let settings_uniforms = world.resource::<ComponentUniforms<PostProcessSettings>>();
    let Some(settings_binding) = settings_uniforms.uniforms().binding() else {
      return Ok(());
    };

    // This will start a new "post process write", obtaining two texture
    // views from the view target - a `source` and a `destination`.
    // `source` is the "current" main texture and you _must_ write into
    // `destination` because calling `post_process_write()` on the
    // [`ViewTarget`] will internally flip the [`ViewTarget`]'s main
    // texture to the `destination` texture. Failing to do so will cause
    // the current main texture information to be lost.
    let post_process = view_target.post_process_write();

    // The bind_group gets created each frame.
    //
    // Normally, you would create a bind_group in the Queue set,
    // but this doesn't work with the post_process_write().
    // The reason it doesn't work is because each post_process_write will alternate the source/destination.
    // The only way to have the correct source/destination for the bind_group
    // is to make sure you get it during the node execution.
    let bind_group = render_context.render_device().create_bind_group(
      "post_process_bind_group",
      &post_process_pipeline.layout,
      // It's important for this to match the BindGroupLayout defined in the PostProcessPipeline
      &BindGroupEntries::sequential((
        // Make sure to use the source view
        post_process.source,
        // Use the sampler created for the pipeline
        &post_process_pipeline.sampler,
        // Set the settings binding
        settings_binding.clone(),
      ))
    );

    // Begin the render pass
    let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
      label: Some("post_process_pass"),
      color_attachments: &[
        Some(RenderPassColorAttachment {
          // We need to specify the post process destination view here
          // to make sure we write to the appropriate texture.
          view: post_process.destination,
          resolve_target: None,
          ops: Operations::default(),
        }),
      ],
      depth_stencil_attachment: None,
      timestamp_writes: None,
      occlusion_query_set: None,
    });

    // This is mostly just wgpu boilerplate for drawing a fullscreen triangle,
    // using the pipeline/bind_group created above
    render_pass.set_render_pipeline(pipeline);
    // By passing in the index of the post process settings on this view, we ensure
    // that in the event that multiple settings were sent to the GPU (as would be the
    // case with multiple cameras), we use the correct one.
    render_pass.set_bind_group(0, &bind_group, &[settings_index.index()]);
    render_pass.draw(0..3, 0..1);

    Ok(())
  }
}

// This contains global data used by the render pipeline. This will be created once on startup.
#[derive(Resource)]
struct PostProcessPipeline {
  layout: BindGroupLayout,
  sampler: Sampler,
  pipeline_id: CachedRenderPipelineId,
}

impl FromWorld for PostProcessPipeline {
  fn from_world(world: &mut World) -> Self {
    let render_device = world.resource::<RenderDevice>();

    // We need to define the bind group layout used for our pipeline
    let layout = render_device.create_bind_group_layout(
      "post_process_bind_group_layout",
      &BindGroupLayoutEntries::sequential(
        // The layout entries will only be visible in the fragment stage
        ShaderStages::FRAGMENT,
        (
          // The screen texture
          texture_2d(TextureSampleType::Float { filterable: true }),
          // The sampler that will be used to sample the screen texture
          sampler(SamplerBindingType::Filtering),
          // The settings uniform that will control the effect
          uniform_buffer::<PostProcessSettings>(true),
        )
      )
    );

    // We can create the sampler here since it won't change at runtime and doesn't depend on the view
    let sampler = render_device.create_sampler(&SamplerDescriptor::default());

    // Get the shader handle
    let shader = world.load_asset(SHADER_ASSET_PATH);

    let pipeline_id = world
      .resource_mut::<PipelineCache>()
      // This will add the pipeline to the cache and queue its creation
      .queue_render_pipeline(RenderPipelineDescriptor {
        label: Some("post_process_pipeline".into()),
        layout: vec![layout.clone()],
        // This will setup a fullscreen triangle for the vertex state
        vertex: fullscreen_shader_vertex_state(),
        fragment: Some(FragmentState {
          shader,
          shader_defs: vec![],
          // Make sure this matches the entry point of your shader.
          // It can be anything as long as it matches here and in the shader.
          entry_point: "fragment".into(),
          targets: vec![
            Some(ColorTargetState {
              format: TextureFormat::bevy_default(),
              blend: None,
              write_mask: ColorWrites::ALL,
            })
          ],
        }),
        // All of the following properties are not important for this effect so just use the default values.
        // This struct doesn't have the Default trait implemented because not all fields can have a default value.
        primitive: PrimitiveState::default(),
        depth_stencil: None,
        multisample: MultisampleState::default(),
        push_constant_ranges: vec![],
        zero_initialize_workgroup_memory: false,
      });

    Self {
      layout,
      sampler,
      pipeline_id,
    }
  }
}

// This is the component that will get passed to the shader
#[derive(Component, Default, Clone, Copy, ExtractComponent, ShaderType)]
struct PostProcessSettings {
  intensity: f32,
  // WebGL2 structs must be 16 byte aligned.
  // #[cfg(feature = "webgl2")]
  // _webgl2_padding: Vec3,
}

#[derive(Resource)]
struct SoundtrackPlayer {
  track_list: Vec<Handle<AudioSource>>,
}

impl SoundtrackPlayer {
  fn new(track_list: Vec<Handle<AudioSource>>) -> Self {
    Self { track_list }
  }
}

#[derive(Component)]
struct FadeIn;

fn main() {
  dbgln!("App stating...");

  App::new()
    // Enable physics
    // .add_plugins((PanOrbitCameraPlugin,))
    // .insert_resource(ClearColor(Color::from(BLUE_200)))
    // .insert_resource(WindowDescriptor {
    //   present_mode: PresentMode::AutoVsync,
    //   ..default()
    // })
    // .add_plugins(
    //   ScheduleRunnerPlugin::run_loop(
    //     // Run 60 times per second.
    //     Duration::from_secs_f64(1.0 / FARERATE_LIMIT)
    //     // Duration::from_secs_f64(10.0)
    //   )
    // )
    .add_plugins((
      // AssetPlugin::default(),
      // AudioPlugin::default(),
      // LogDiagnosticsPlugin::default(),
      // PostProcessPlugin,
      DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
          position: WindowPosition::At(IVec2::new(1200, 0)),
          // title: "Bevy Game".to_string(),
          resolution: WindowResolution::new(
            // WP_W / WP_SCALE,
            // WP_H / WP_SCALE
            USE_WIN_SIZE.x,
            USE_WIN_SIZE.y
          ).with_scale_factor_override(1.0),
          present_mode: PresentMode::AutoNoVsync,
          // present_mode: PresentMode::AutoVsync,
          // present_mode: PresentMode::Immediate,
          // mode: Fullscreen(MonitorSelection::Primary),
          // mode: BorderlessFullscreen(MonitorSelection::Primary),
          // resizable: false,
          // fit_canvas_to_parent: true,
          // fullsize_content_view: true,
          ..Default::default()
        }),
        ..Default::default()
      }), // .set(WindowPlugin {}),
      PhysicsPlugins::default(),
      PostProcessPlugin,
      // PhysicsDebugPlugin::default(),
      asset_loader::MAssetLoaderPlugin,
      cubes::CubesPlugin,
      debug::DebugPlugin,
      camera::CameraPlugin,
      lights::MLightsPlugin,
      terrain::MTerrainPlugin,
      sky::MSkyPlugin,
      entities::base::MEntityBasePlugin,
      entities::with_children::MEntityWithChildrenPlugin,
      state::MGameStatePlugin,
    ))
    // .insert_gizmo_config(
    //   PhysicsGizmos {
    //     aabb_color: Some(Color::WHITE),
    //     ..default()
    //   },
    //   GizmoConfig::default()
    // )
    .add_systems(Startup, setup)
    .add_systems(Update, update.run_if(on_timer(Duration::from_millis(1000))))
    .insert_resource(Gravity(physics::get_gravity_vec3()))
    .run();
}

// prettier-ignore
fn setup(
  mut res_mut_audio_cache: Option<ResMut</*res_mut_texture_cache::*/AudioCache>>,
  asset_server: Res<AssetServer>,
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>
) {

  let audio_hashmap: &mut ResMut<AudioCache> = res_mut_audio_cache.as_mut().unwrap();
  // let track_1: Handle<AudioSource> = asset_server.load::<AudioSource>(sys_paths::sounds::EPaths::EnvOne.as_str());
  let track_1 = cache_load_audio(
    audio_hashmap, 
    &asset_server, 
    EAudio::EnvOne.as_str(),
    false
  );

  // all options are same as default

  commands.spawn((
    AudioPlayer(track_1),
    PlaybackSettings {
      mode: PlaybackMode::Loop,
      volume: Volume::default(),
      ..default()
    },
    // FadeIn,
  ));

  // commands.spawn(AudioPlayer::new(track_1 ));
  // let audio  = AudioPlayer::new(track_1);
  // commands.spawn(audio);

}

// prettier-ignore
fn update() {}
