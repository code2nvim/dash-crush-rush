use super::{cfg::player::*, *};

use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player).add_systems(
            Update,
            (move_player, leap_player, rotate_player, despawn_player),
        );
    }
}

#[derive(Component)]
#[component(immutable)]
pub struct Player;

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn((
            Player,
            (
                Velocity(0.0),
                Direction(Vec3::new(0.0, 0.0, 0.0)),
                Shield { active: false },
            ),
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

fn leap_player(
    time: Res<Time>,
    key: Res<ButtonInput<KeyCode>>,
    mut transform: Single<&mut Transform, With<Player>>,
    mut velocity: Single<&mut Velocity, With<Player>>,
) {
    if key.pressed(cfg::bind::JUMP) && transform.translation.y == default::RADIUS {
        velocity.0 = gravity::START;
    } else {
        velocity.0 -= gravity::FALL;
    }
    let position = transform.translation.y + velocity.0 * time.delta_secs();
    if position < default::RADIUS {
        transform.translation.y = default::RADIUS;
        velocity.0 = 0.0;
    } else {
        transform.translation.y = position;
    }
}

fn move_player(
    time: Res<Time>,
    key: Res<ButtonInput<KeyCode>>,
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
    transform.translation += direction.normalize_or_zero() * default::SPEED * time.delta_secs();
}

fn rotate_player(
    cursor: Res<Cursor>,
    mut transform: Single<&mut Transform, With<Player>>,
    mut direction: Single<&mut Direction, With<Player>>,
) {
    let dir = cursor.0 - transform.translation;
    direction.0 = Vec3::new(dir.x, 0.0, dir.z).normalize_or_zero();
    transform.rotation = Quat::from_rotation_y(dir.x.atan2(dir.z));
}

// TODO: actually despawn player (currently just resetting position)
fn despawn_player(
    mut destroy: EventReader<DestroyPlayer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut material: Single<&mut MeshMaterial3d<StandardMaterial>, With<Player>>,
    mut player: Single<&mut Transform, With<Player>>,
) {
    for destroy in destroy.read() {
        player.translation = (0.0, cfg::player::default::RADIUS, 0.0).into();
        match destroy.0 {
            PlayerReason::Enemy => material.0 = materials.add(cfg::enemy::COLOR),
            PlayerReason::Obstacle => material.0 = materials.add(cfg::obstacle::COLOR),
            PlayerReason::Wall => material.0 = materials.add(cfg::wall::COLOR),
        }
    }
}
