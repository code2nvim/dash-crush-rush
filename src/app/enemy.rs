use super::{cfg::enemy::*, *};

use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy;

#[derive(Resource)]
pub struct Casual {
    pub corner: bool,
    pub timer: Timer,
}

pub fn spawn_enemy(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut casual: ResMut<Casual>,
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

pub fn move_enemy(
    mut enemies: Query<&mut Transform, With<Enemy>>,
    player: Single<&Transform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
) {
    for mut enemy in &mut enemies {
        let direction = (player.translation - enemy.translation).normalize_or_zero();
        enemy.translation += direction * SPEED * time.delta_secs();
    }
}

pub fn despawn_enemy(
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
