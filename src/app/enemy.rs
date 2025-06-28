use super::cfg::enemy::*;

use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy;

pub fn spawn_enemy(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let size = crate::app::cfg::ground::SIZE * 0.5;
    let positions = [(-size, -size), (-size, size), (size, -size), (size, size)];
    for pos in positions {
        commands.spawn((
            Enemy,
            (
                Mesh3d(meshes.add(Sphere::new(RADIUS))),
                MeshMaterial3d(materials.add(COLOR)),
                Transform::from_xyz(pos.0, RADIUS, pos.1),
            ),
        ));
    }
}
