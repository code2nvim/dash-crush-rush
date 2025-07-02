use super::cfg::enemy::*;

use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy(Vec3);

pub fn spawn_enemy(
    enemies: Query<&Enemy>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // TODO: add more variety
    if enemies.iter().count() == 4 {
        return;
    }
    let border = crate::app::cfg::ground::SIZE * 0.5;
    let positions = [
        (border, 0.0),
        (-border, 0.0),
        (0.0, border),
        (0.0, -border),
    ];
    for pos in positions {
        commands.spawn((
            Enemy(Vec3::new(0.0 - pos.0, 0.0, 0.0 - pos.1).normalize_or_zero()),
            (
                Mesh3d(meshes.add(Sphere::new(RADIUS))),
                MeshMaterial3d(materials.add(COLOR)),
                Transform::from_xyz(pos.0, RADIUS, pos.1),
            ),
        ));
    }
}

pub fn move_enemy(time: Res<Time>, mut enemies: Query<(&mut Enemy, &mut Transform)>) {
    for (mut enemy, mut transform) in &mut enemies {
        let border = crate::app::cfg::ground::SIZE * 0.5;
        let pos = transform.translation + enemy.0 * SPEED * time.delta_secs();
        if pos.x >= border || pos.x <= -border || pos.z >= border || pos.z <= -border {
            enemy.0 = -enemy.0;
        } else {
            transform.translation = pos;
        }
    }
}
