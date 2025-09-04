use super::{cfg::obstacle::*, *};

use bevy::prelude::*;

pub struct ObstaclePlugin;

impl Plugin for ObstaclePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_obstacle)
            .add_systems(Update, (destroy_player, move_obstacle));
    }
}

#[derive(Component)]
#[component(immutable)]
struct Obstacle;

fn spawn_obstacle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Obstacle,
        (
            Mesh3d(meshes.add(Cuboid::new(cfg::ground::SIZE, 1.0, LENGTH))),
            MeshMaterial3d(materials.add(COLOR)),
            Transform::from_xyz(0.0, 0.0, -cfg::boundary::SIZE),
        ),
    ));
}

fn move_obstacle(time: Res<Time>, mut obstacle: Single<&mut Transform, With<Obstacle>>) {
    obstacle.translation.z += SPEED * time.delta_secs();
    if obstacle.translation.z >= cfg::ground::SIZE {
        obstacle.translation.z = -cfg::ground::SIZE;
    }
}

fn destroy_player(
    player: Single<&Transform, With<Player>>,
    obstacle: Single<&Transform, With<Obstacle>>,
    mut destroy: EventWriter<DestroyPlayer>,
) {
    use cfg::player::default::RADIUS;
    let distance = (player.translation.z - obstacle.translation.z).abs();
    if distance <= RADIUS && player.translation.y == RADIUS {
        destroy.write(DestroyPlayer(PlayerReason::Obstacle));
    }
}
