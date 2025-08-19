use super::*;

use bevy::prelude::*;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Cursor((0.0, 0.0, 0.0).into()))
            .add_systems(Update, (update_cursor, draw_cursor));
    }
}

#[derive(Resource)]
pub struct Cursor(pub Vec3);

fn update_cursor(
    window: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform)>,
    ground: Single<&GlobalTransform, With<Ground>>,
    mut cursor: ResMut<Cursor>,
) {
    if let Some(position) = window.cursor_position()
        && let Ok(ray) = camera.0.viewport_to_world(camera.1, position)
        && let Some(distance) =
            ray.intersect_plane(ground.translation(), InfinitePlane3d::new(ground.up()))
    {
        cursor.0 = ray.get_point(distance);
    }
}

fn draw_cursor(mut gizmos: Gizmos, cursor: Res<Cursor>, ground: Single<&Transform, With<Ground>>) {
    gizmos.circle(
        Isometry3d::new(
            cursor.0 + ground.up() * 0.01,
            Quat::from_rotation_arc(Vec3::Z, ground.up().as_vec3()),
        ),
        cfg::bullet::RADIUS,
        cfg::player::gear::COLOR,
    );
}
