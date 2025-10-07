use super::{cfg::wall::*, *};

use bevy::prelude::*;

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_wall)
            .add_systems(Update, (destroy_player, move_wall));
    }
}

#[derive(Component)]
#[component(immutable)]
struct Wall;

fn spawn_wall(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Wall,
        (
            Mesh3d(meshes.add(Cuboid::new(cfg::ground::SIZE, cfg::ground::SIZE / 4.0, 0.1))),
            MeshMaterial3d(materials.add(COLOR)),
            Transform::from_xyz(0.0, cfg::ground::SIZE / 8.0, -cfg::boundary::SIZE),
        ),
    ));
}

fn move_wall(time: Res<Time>, mut wall: Single<&mut Transform, With<Wall>>) {
    wall.translation.z += SPEED * time.delta_secs();
    if wall.translation.z >= cfg::ground::SIZE {
        wall.translation.z = -cfg::boundary::SIZE;
    }
}

fn destroy_player(
    player: Single<(&Shield, &Transform), With<Player>>,
    wall: Single<&Transform, With<Wall>>,
    mut destroy: EventWriter<DestroyPlayer>,
) {
    use cfg::player::default::RADIUS;
    let (shield, transform) = *player;
    let distance = (transform.translation.z - wall.translation.z).abs();
    if !shield.active && distance <= RADIUS {
        destroy.write(DestroyPlayer(PlayerReason::Wall));
    }
}
