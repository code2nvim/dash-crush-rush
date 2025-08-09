use super::{cfg::bullet::*, *};

use bevy::prelude::*;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, ((spawn_bullet, move_bullet, despawn_bullet),))
            .insert_resource(Bullets {
                first: true,
                timer: Timer::from_seconds(cfg::bullet::INTERVAL, TimerMode::Repeating),
            });
    }
}

#[derive(Component)]
#[component(immutable)]
pub struct Bullet(Vec3);

#[derive(Resource)]
struct Bullets {
    first: bool,
    timer: Timer,
}

fn spawn_bullet(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut fire: ResMut<Bullets>,
    time: Res<Time>,
    mouse: Res<ButtonInput<MouseButton>>,
    player: Single<(&Player, &Transform)>,
) {
    if !mouse.pressed(cfg::bind::FIRE) {
        return if fire.timer.tick(time.delta()).finished() {
            fire.first = true;
            fire.timer.reset();
        };
    }
    if fire.first || fire.timer.tick(time.delta()).finished() {
        fire.first = false;
        fire.timer.reset();
        let (player, transform) = player.into_inner();
        let pos = transform.translation;
        commands.spawn((
            Bullet(player.direction.normalize_or_zero()),
            (
                Mesh3d(meshes.add(Sphere::new(RADIUS))),
                MeshMaterial3d(materials.add(COLOR)),
                Transform::from_xyz(pos.x, pos.y, pos.z),
            ),
        ));
    }
}

fn move_bullet(mut bullets: Query<(&Bullet, &mut Transform)>, time: Res<Time>) {
    for (bullet, mut transform) in &mut bullets {
        transform.translation += bullet.0 * SPEED * time.delta_secs();
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
