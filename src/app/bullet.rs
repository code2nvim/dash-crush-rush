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
    mut timer: ResMut<Fire>,
    time: Res<Time>,
    mouse: Res<ButtonInput<MouseButton>>,
    player: Single<(&Player, &Transform)>,
) {
    if !mouse.pressed(cfg::bind::FIRE) {
        timer.first = true;
        return timer.timer.reset();
    }
    if timer.timer.tick(time.delta()).finished() || timer.first {
        timer.first = false;
        let (player, transform) = player.into_inner();
        let pos = transform.translation;
        commands.spawn((
            Bullet(player.direction.normalize_or_zero() * SPEED * time.delta_secs()),
            (
                Mesh3d(meshes.add(Sphere::new(RADIUS))),
                MeshMaterial3d(materials.add(COLOR)),
                Transform::from_xyz(pos.x, pos.y, pos.z),
            ),
        ));
    }
}

pub fn drive_bullet(mut bullets: Query<(&mut Bullet, &mut Transform)>) {
    for (bullet, mut transform) in &mut bullets {
        transform.translation += bullet.0;
    }
}
