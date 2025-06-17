use super::cfg::camera::*;

use bevy::prelude::*;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(default::POS.0, default::POS.1, default::POS.2)
            .looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
