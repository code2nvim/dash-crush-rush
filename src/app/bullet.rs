use super::{cfg::bullet::*, *};

use bevy::prelude::*;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Fire>()
            .add_systems(
                Update,
                (
                    fire_bullet,
                    spawn_bullet,
                    move_bullet,
                    destroy_enemy,
                    despawn_bullet,
                ),
            )
            .insert_resource(Bullets {
                timer: Timer::from_seconds(cfg::bullet::INTERVAL, TimerMode::Once),
            });
    }
}

#[derive(Event)]
pub struct Fire;

#[derive(Component)]
#[component(immutable)]
pub struct Bullet(Vec3);

#[derive(Resource)]
struct Bullets {
    timer: Timer,
}

fn fire_bullet(
    time: Res<Time>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut bullets: ResMut<Bullets>,
    mut fire: EventWriter<Fire>,
) {
    bullets.timer.tick(time.delta());
    if !mouse.pressed(cfg::bind::FIRE) {
        return;
    }
    if bullets.timer.finished() {
        fire.write(Fire);
        bullets.timer.reset();
    }
}

fn spawn_bullet(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut fire: EventReader<Fire>,
    direction: Single<&Direction>,
    transform: Single<&Transform, With<Player>>,
) {
    for _ in fire.read() {
        let pos = transform.translation;
        commands.spawn((
            Bullet(direction.0.normalize_or_zero()),
            (
                Mesh3d(meshes.add(Sphere::new(RADIUS))),
                MeshMaterial3d(materials.add(COLOR)),
                Transform::from_xyz(pos.x, pos.y, pos.z),
            ),
        ));
    }
}

fn move_bullet(time: Res<Time>, mut bullets: Query<(&Bullet, &mut Transform)>) {
    for (bullet, mut transform) in &mut bullets {
        transform.translation += bullet.0 * SPEED * time.delta_secs();
    }
}

fn destroy_enemy(
    bullets: Query<&Transform, With<Bullet>>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
    mut destroy: EventWriter<DestroyEnemy>,
) {
    for (entity, transform) in enemies {
        for bullet in bullets {
            let distance = RADIUS + cfg::enemy::RADIUS;
            if transform.translation.distance(bullet.translation) <= distance {
                destroy.write(DestroyEnemy(entity));
            }
        }
    }
}

fn despawn_bullet(mut commands: Commands, bullets: Query<(Entity, &Transform), With<Bullet>>) {
    for (entity, transform) in bullets {
        let pos = transform.translation;
        let border = crate::app::cfg::boundary::SIZE * 0.5;
        if pos.x >= border || pos.x <= -border || pos.z >= border || pos.z <= -border {
            commands.entity(entity).despawn();
        }
    }
}
