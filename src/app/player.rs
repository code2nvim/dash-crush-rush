use bevy::prelude::*;

use super::cfg::player::default::*;

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
            Mesh3d(meshes.add(Sphere::new(RADIUS))),
            MeshMaterial3d(materials.add(COLOR)),
            Transform::from_xyz(0.0, 0.0, 0.0),
        ),
    ));
}
