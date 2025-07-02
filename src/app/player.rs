use super::{cfg::player::*, *};

use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub direction: Vec3,
}

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn((
            Player {
                direction: Vec3::new(0.0, 0.0, 0.0),
            },
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
    mut player: Single<&mut Transform, With<Player>>,
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
    player.translation += direction.normalize_or_zero() * default::SPEED * time.delta_secs();
}

pub fn reset_player(
    key: Res<ButtonInput<KeyCode>>,
    mut player: Single<&mut Transform, With<Player>>,
) {
    if key.pressed(cfg::bind::RESET) {
        player.translation = (0.0, default::RADIUS, 0.0).into();
    }
}

pub fn rotate_player(
    window: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform)>,
    ground: Single<&GlobalTransform, With<Ground>>,
    player: Single<(&mut Player, &mut Transform)>,
) {
    if let Some(position) = window.cursor_position()
        && let Ok(ray) = camera.0.viewport_to_world(camera.1, position)
        && let Some(distance) =
            ray.intersect_plane(ground.translation(), InfinitePlane3d::new(ground.up()))
    {
        let (mut player, mut transform) = player.into_inner();
        let direction = ray.get_point(distance) - transform.translation;
        player.direction = direction;
        transform.rotation = Quat::from_rotation_y(direction.x.atan2(direction.z));
    }
}

// TODO: actually destroy player (currently just resetting position)
pub fn destroy_player(
    enemies: Query<&Transform, With<Enemy>>,
    mut player: Single<&mut Transform, (With<Player>, Without<Enemy>)>,
) {
    for enemy in enemies {
        if player.translation.distance(enemy.translation) <= default::RADIUS + cfg::enemy::RADIUS {
            player.translation = (0.0, default::RADIUS, 0.0).into();
        }
    }
}
