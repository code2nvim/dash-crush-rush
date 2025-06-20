use super::*;

use bevy::prelude::*;

#[derive(Component)]
pub struct Cursor(pub Vec3);

pub fn spawn_cursor(mut commands: Commands) {
    commands.spawn(Cursor(Vec3::new(0.0, 0.0, 0.0)));
}

pub fn locate_cursor(
    camera: Single<(&Camera, &GlobalTransform)>,
    ground: Single<&GlobalTransform, With<Ground>>,
    window: Single<&Window>,
    mut cursor: Single<&mut Cursor>,
) {
    let (camera, transform) = *camera;
    let Some(position) = window.cursor_position() else {
        return;
    };
    let Ok(ray) = camera.viewport_to_world(transform, position) else {
        return;
    };
    let Some(distance) =
        ray.intersect_plane(ground.translation(), InfinitePlane3d::new(ground.up()))
    else {
        return;
    };
    cursor.0 = ray.get_point(distance);
}
