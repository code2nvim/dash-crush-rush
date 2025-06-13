use super::cfg::player::*;

use bevy::prelude::*;

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
            Mesh3d(meshes.add(Sphere::new(default::RADIUS))),
            MeshMaterial3d(materials.add(default::COLOR)),
            Transform::from_xyz(0.0, 0.0, 0.0),
        ),
    ));
}
pub fn move_player(
    key: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut transform: Query<&mut Transform, With<Player>>,
) {
    let mut direction = Vec3::ZERO;
    if key.pressed(KeyCode::KeyW) {
        direction += Vec3::new(0.0, 0.0, -1.0);
    }
    if key.pressed(KeyCode::KeyA) {
        direction += Vec3::new(-1.0, 0.0, 0.0);
    }
    if key.pressed(KeyCode::KeyS) {
        direction += Vec3::new(0.0, 0.0, 1.0);
    }
    if key.pressed(KeyCode::KeyD) {
        direction += Vec3::new(1.0, 0.0, 0.0);
    }
    let Ok(mut transform) = transform.single_mut() else {
        return;
    };
    transform.translation += direction * time.delta_secs() * 10.0;
}
