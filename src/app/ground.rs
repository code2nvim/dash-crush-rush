use super::cfg::ground::*;

use bevy::prelude::*;

#[derive(Component)]
#[component(immutable)]
pub struct Ground;

pub fn spawn_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Ground,
        (
            Mesh3d(meshes.add(Plane3d::default().mesh().size(SIZE, SIZE))),
            MeshMaterial3d(materials.add(COLOR)),
        ),
    ));
}
