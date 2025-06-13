use bevy::prelude::*;

use super::cfg::camera::default::*;

#[derive(Component)]
pub struct Camera;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera,
        (
            Camera3d::default(),
            Transform::from_xyz(POS.0, POS.1, POS.2).looking_at(Vec3::ZERO, Vec3::Y),
        ),
    ));
}
