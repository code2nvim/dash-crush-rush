use bevy::prelude::*;

use super::cfg::ground::default::*;

#[derive(Component)]
pub struct Ground;

pub fn spawn_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Ground,
        (
            Mesh3d(meshes.add(Plane3d::default().mesh().size(SIZE.0, SIZE.1))),
            MeshMaterial3d(materials.add(COLOR)),
        ),
    ));
}
