use avian3d::prelude::{ ExternalForce, ExternalImpulse, GravityScale };
use bevy::math::Vec3;

use crate::GRAVITY;

// prettier-ignore
pub fn get_external_impulse(impulse3: Vec3, is_persistent: bool) -> ExternalImpulse {
  let mut impuse = ExternalImpulse::default();
  impuse
    .apply_impulse_at_point(
      impulse3, 
      Vec3::ZERO, 
      Vec3::ZERO)
    .with_persistence(is_persistent);

  impuse
}

// prettier-ignore
pub fn get_external_force(impulse3: Vec3, is_persistent: bool) -> ExternalForce {
  let mut force = ExternalForce::default();
  force
    .apply_force_at_point(
      impulse3, 
      Vec3::ZERO, 
      Vec3::ZERO)
    .with_persistence(is_persistent);

  force
}

pub fn get_gravity() -> f32 {
  GRAVITY
}

pub fn get_gravity_vec3() -> Vec3 {
  // Vec3::splat(GRAVITY)
  Vec3::NEG_Y * GRAVITY
}

pub fn get_gravity_scale(scale: f32) -> GravityScale {
  GravityScale(scale)
}
