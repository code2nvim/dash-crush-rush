use super::{cfg::enemy::*, *};

use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy;

#[derive(Resource)]
pub struct Casual {
    pub vertical: bool,
    pub timer: Timer,
}

pub fn spawn_enemy(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut casual: ResMut<Casual>,
    time: Res<Time>,
) {
    if casual.timer.tick(time.delta()).finished() {
        let border = crate::app::cfg::ground::SIZE * 0.5;
        let positions = if casual.vertical {
            [(border, border), (-border, border)]
        } else {
            [(border, -border), (-border, -border)]
        };
        casual.vertical = !casual.vertical;
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
    /*
    for (mut enemy, mut transform) in &mut enemies {
        let border = crate::app::cfg::ground::SIZE * 0.5;
        let pos = transform.translation + enemy.0 * SPEED * time.delta_secs();
        if pos.x >= border || pos.x <= -border || pos.z >= border || pos.z <= -border {
            enemy.0 = -enemy.0;
        } else {
            transform.translation = pos;
        }
    }
    */
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
