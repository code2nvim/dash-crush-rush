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
            timer: Timer::from_seconds(cfg::enemy::INTERVAL, TimerMode::Repeating),
        });
    }
}

#[derive(Component)]
#[component(immutable)]
pub struct Enemy;

#[derive(Resource)]
struct Enemies {
    corner: bool,
    timer: Timer,
}

fn spawn_enemy(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut casual: ResMut<Enemies>,
    time: Res<Time>,
    player: Single<&Transform, With<Player>>,
) {
    if casual.timer.tick(time.delta()).finished() {
        let size = cfg::ground::SIZE * 0.5;
        let positions: Vec<(f32, f32)> = if casual.corner {
            [(-size, -size), (size, -size)].into()
        } else {
            [
                (player.translation.x, -size),
                (-size, player.translation.z),
                (size, player.translation.z),
            ]
            .into()
        };
        casual.corner = !casual.corner;
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
    mut enemies: Query<&mut Transform, With<Enemy>>,
    player: Single<&Transform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
) {
    for mut enemy in &mut enemies {
        let relative = player.translation - enemy.translation;
        let direction = Vec3::new(relative.x, 0.0, relative.z).normalize_or_zero();
        enemy.translation += direction * SPEED * time.delta_secs();
    }
}

fn despawn_enemy(
    mut commands: Commands,
    enemies: Query<(Entity, &Transform), (With<Enemy>, Without<Bullet>)>,
    bullets: Query<&Transform, With<Bullet>>,
) {
    for (entity, enemy) in enemies {
        for bullet in bullets {
            if enemy.translation.distance(bullet.translation) <= RADIUS + cfg::bullet::RADIUS {
                commands.entity(entity).despawn();
            }
        }
    }
}

fn destroy_player(
    player: Single<&Transform, With<Player>>,
    enemies: Query<&Transform, With<Enemy>>,
    mut destroy: EventWriter<Destroy>,
) {
    for enemy in enemies {
        let distance = player.translation.distance(enemy.translation);
        if distance <= cfg::player::default::RADIUS + cfg::enemy::RADIUS {
            destroy.write(Destroy(Reason::Enemy));
        }
    }
}
