use bevy::prelude::*;

use super::cfg::player::*;

#[derive(Component)]
pub struct Player;

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Player,
        (
            Mesh3d(meshes.add(Sphere::new(default::RADIUS))),
            MeshMaterial3d(materials.add(default::COLOR)),
            Transform::from_xyz(0.0, 0.0, 0.0),
        ),
    ));
}
