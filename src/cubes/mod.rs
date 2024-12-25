use bevy::prelude::*;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct CubeMarker;

#[derive(Resource, Default)]
pub struct CubePosition(Vec3);

pub struct CubesPlugin;
impl Plugin for CubesPlugin {
  fn build(&self, app: &mut App) {
    app
      .register_type::<CubeMarker>()
      .init_resource::<CubePosition>()
      .add_systems(Startup, startup)
      // .add_systems(Update, setup.run_if(in_state(GameState::Loading)))
      // .add_systems(OnEnter(GameState::Game), enter_game)
      // .add_systems(Update, recycle.run_if(in_state(GameState::Game)));
      .add_systems(Update, update);
  }
}

// prettier-ignore
fn startup(
  // mut commands: Commands,
  // mut meshes: ResMut<Assets<Mesh>>,
  // mut cube_positions: ResMut<CubePosition>
) {}

// prettier-ignore
fn update(
  // mut commands: Commands,
  // mut cube_positions: ResMut<CubePosition>,
  time: Res<Time>,
  query: Query<(Entity, &CubeMarker)>
) {
  // for (entity, _cube_marker) in query.iter() {
  //     let mut position = cube_positions.0;
  //     position.x += time.delta_seconds();
  //     cube_positions.0 = position;
  //     commands.entity(entity).insert(position);
  // }
}
