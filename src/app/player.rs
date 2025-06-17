use super::cfg::player::*;

use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn((
            Player,
            (
                Mesh3d(meshes.add(Sphere::new(default::RADIUS))),
                MeshMaterial3d(materials.add(default::COLOR)),
                Transform::from_xyz(0.0, 0.0, 0.0),
            ),
        ))
        .with_children(|parent| {
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(gear::SIZE.0, gear::SIZE.1, gear::SIZE.2))),
                MeshMaterial3d(materials.add(gear::COLOR)),
                Transform::from_xyz(gear::POS.0, gear::POS.1, gear::POS.2),
            ));
        });
}

pub fn move_player(
    key: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut transform: Single<&mut Transform, With<Player>>,
) {
    let distance = time.delta_secs() * default::SPEED;
    if key.pressed(KeyCode::KeyW) {
        transform.translation.z -= distance;
    }
    if key.pressed(KeyCode::KeyA) {
        transform.translation.x -= distance;
    }
    if key.pressed(KeyCode::KeyS) {
        transform.translation.z += distance;
    }
    if key.pressed(KeyCode::KeyD) {
        transform.translation.x += distance;
    }
}
