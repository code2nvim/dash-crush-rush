use super::{cfg::bullet::*, *};

use bevy::prelude::*;

#[derive(Component)]
pub struct Bullet;

#[derive(Resource)]
pub struct FireTimer(pub Timer);

pub fn fire_bullet(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut timer: ResMut<FireTimer>,
    time: Res<Time>,
    mouse: Res<ButtonInput<MouseButton>>,
    player: Single<&GlobalTransform, With<Player>>,
) {
    if !mouse.pressed(cfg::bind::FIRE) {
        return timer.0.reset();
    }
    if timer.0.tick(time.delta()).just_finished() {
        let player = player.translation();
        commands.spawn((
            Bullet,
            (
                Mesh3d(meshes.add(Sphere::new(RADIUS))),
                MeshMaterial3d(materials.add(COLOR)),
                Transform::from_xyz(player.x, player.y, player.z),
            ),
        ));
    }
}
