use bevy::prelude::*;

#[derive(Component, Debug, PartialEq, Eq)]
pub struct PhysicsStaticObject;

#[derive(Component, Debug, PartialEq, Eq)]
pub struct PhysicsStaticObjectTerrain;

#[derive(Component, Debug, PartialEq, Eq)]
pub struct PhysicsDynamicObject;

#[derive(Component, Debug, PartialEq, Eq)]
pub struct PhysicsDynamicObjectFloatable;
