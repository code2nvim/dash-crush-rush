use super::{cfg::enemy::*, *};

use bevy::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_enemy, move_enemy, despawn_enemy, destroy_player),
        )
        .insert_resource(Enemies {
            corner: true,
            interval: Timer::from_seconds(cfg::enemy::INTERVAL, TimerMode::Repeating),
        });
    }
}

#[derive(Component)]
#[component(immutable)]
pub struct Enemy;

#[derive(Resource)]
struct Enemies {
    corner: bool,
    interval: Timer,
}

fn spawn_enemy(
    time: Res<Time>,
    player: Single<&Transform, With<Player>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut enemies: ResMut<Enemies>,
) {
    if enemies.interval.tick(time.delta()).finished() {
        let size = cfg::ground::SIZE * 0.5;
        let positions: Vec<(f32, f32)> = if enemies.corner {
            [(-size, -size), (size, -size)].into()
        } else {
            [
                (-size, player.translation.z),
                (size, player.translation.z),
                (player.translation.x, -size),
            ]
            .into()
        };
        enemies.corner = !enemies.corner;
        for pos in positions {
            commands.spawn((
                Enemy,
                (
                    Mesh3d(meshes.add(Sphere::new(RADIUS))),
                    MeshMaterial3d(materials.add(COLOR)),
                    Transform::from_xyz(pos.0, RADIUS, pos.1),
                ),
            ));
        }
    }
}

fn move_enemy(
    time: Res<Time>,
    player: Single<&Transform, (With<Player>, Without<Enemy>)>,
    mut enemies: Query<&mut Transform, With<Enemy>>,
) {
    for mut enemy in &mut enemies {
        let relative = player.translation - enemy.translation;
        let direction = Vec3::new(relative.x, 0.0, relative.z).normalize_or_zero();
        enemy.translation += direction * SPEED * time.delta_secs();
    }
}

fn destroy_player(
    player: Single<&Transform, With<Player>>,
    enemies: Query<&Transform, With<Enemy>>,
    mut destroy: EventWriter<DestroyPlayer>,
) {
    for enemy in enemies {
        let distance = player.translation.distance(enemy.translation);
        if distance <= cfg::player::default::RADIUS + cfg::enemy::RADIUS {
            destroy.write(DestroyPlayer(PlayerReason::Enemy));
        }
    }
}

fn despawn_enemy(mut commands: Commands, mut enemies: EventReader<DestroyEnemy>) {
    for enemy in enemies.read() {
        commands.entity(enemy.0).try_despawn();
    }
}
