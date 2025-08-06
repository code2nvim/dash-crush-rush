use super::{cfg::player::*, *};

use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player) // cargo fmt
            .add_systems(
                Update,
                (move_player, leap_player, rotate_player, despawn_player),
            );
    }
}

#[derive(Component)]
pub struct Player {
    pub direction: Vec3,
    velocity: f32,
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn((
            Player {
                direction: Vec3::new(0.0, 0.0, 0.0),
                velocity: 0.0,
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

fn leap_player(
    key: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    player: Single<(&mut Player, &mut Transform)>,
) {
    let (mut player, mut transform) = player.into_inner();
    if key.pressed(cfg::bind::JUMP) && transform.translation.y == default::RADIUS {
        player.velocity = 60.0;
    } else {
        player.velocity -= 5.0;
    }
    let pos = transform.translation.y + player.velocity * time.delta_secs();
    if pos < default::RADIUS {
        transform.translation.y = default::RADIUS;
        player.velocity = 0.0;
    } else {
        transform.translation.y = pos;
    }
}

fn move_player(
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

fn rotate_player(
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
        player.direction = Vec3::new(direction.x, 0.0, direction.z).normalize_or_zero();
        transform.rotation = Quat::from_rotation_y(direction.x.atan2(direction.z));
    }
}

// TODO: actually destroy player (currently just resetting position)
fn despawn_player(
    mut destroy: EventReader<Destroy>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut material: Single<&mut MeshMaterial3d<StandardMaterial>, With<Player>>,
    mut player: Single<&mut Transform, With<Player>>,
) {
    for destroy in destroy.read() {
        player.translation = (0.0, cfg::player::default::RADIUS, 0.0).into();
        match destroy.0 {
            Reason::Enemy => material.0 = materials.add(cfg::enemy::COLOR),
        }
    }
}
