use super::{cfg::bullet::*, *};

use bevy::prelude::*;

#[derive(Component)]
pub struct Bullet(Vec3);

#[derive(Resource)]
pub struct Fire {
    pub first: bool,
    pub timer: Timer,
}

pub fn fire_bullet(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut fire: ResMut<Fire>,
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

pub fn drive_bullet(time: Res<Time>, mut bullets: Query<(&Bullet, &mut Transform)>) {
    for (bullet, mut transform) in &mut bullets {
        transform.translation += bullet.0 * SPEED * time.delta_secs();
    }
}

// TODO: destroy_bullet
