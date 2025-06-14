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
    if let Ok(mut transform) = transform.single_mut() {
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
    };
}
