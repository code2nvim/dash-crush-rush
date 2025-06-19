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
    let (mut x, y, mut z): (f32, f32, f32) = (0.0, 0.0, 0.0);
    if key.pressed(cfg::bind::MOV_F) {
        z = -1.0;
    }
    if key.pressed(cfg::bind::MOV_L) {
        x = -1.0;
    }
    if key.pressed(cfg::bind::MOV_B) {
        z += 1.0;
    }
    if key.pressed(cfg::bind::MOV_R) {
        x += 1.0;
    }
    transform.translation += if x.abs() == 1.0 && z.abs() == 1.0 {
        Vec3::new(x, y, z) * default::SPEED * time.delta_secs() / f32::sqrt(2.0)
    } else {
        Vec3::new(x, y, z) * default::SPEED * time.delta_secs()
    };
}

pub fn rotate_player(
    camera: Single<(&Camera, &GlobalTransform)>,
    ground: Single<&GlobalTransform, With<Ground>>,
    window: Single<&Window>,
    mut player: Query<&mut Transform, With<Player>>,
) {
    let (camera, transform) = *camera;
    let Some(position) = window.cursor_position() else {
        return;
    };
    let Ok(ray) = camera.viewport_to_world(transform, position) else {
        return;
    };
    let Ok(mut player) = player.single_mut() else {
        return;
    };
    let Some(distance) =
        ray.intersect_plane(ground.translation(), InfinitePlane3d::new(ground.up()))
    else {
        return;
    };
    let direction = ray.get_point(distance) - player.translation;
    player.rotation = Quat::from_rotation_y(direction.x.atan2(direction.z));
}
