use super::{cfg::player::*, *};

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
                Transform::from_xyz(0.0, default::RADIUS, 0.0),
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
    let mut direction = Vec3::new(0.0, 0.0, 0.0);
    if key.pressed(cfg::bind::MOV_F) {
        direction.z -= 1.0;
    }
    if key.pressed(cfg::bind::MOV_L) {
        direction.x -= 1.0;
    }
    if key.pressed(cfg::bind::MOV_B) {
        direction.z += 1.0;
    }
    if key.pressed(cfg::bind::MOV_R) {
        direction.x += 1.0;
    }
    transform.translation += direction.normalize_or_zero() * default::SPEED * time.delta_secs()
}

pub fn rotate_player(cursor: Res<Cursor>, mut player: Single<&mut Transform, With<Player>>) {
    let direction = cursor.0 - player.translation;
    player.rotation = Quat::from_rotation_y(direction.x.atan2(direction.z));
}
