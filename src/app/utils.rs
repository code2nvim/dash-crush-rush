use bevy::prelude::*;

#[derive(Component)]
pub struct Velocity(pub f32);

#[derive(Component)]
pub struct Direction(pub Vec3);
