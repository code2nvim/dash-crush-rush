use super::cfg::light::*;

use bevy::prelude::*;

pub fn spawn_light(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: BRIGHTNESS,
        ..default()
    });
    commands.spawn((
        DirectionalLight {
            illuminance: ILLUMINANCE,
            shadows_enabled: true,
            ..default()
        },
        Transform::default().looking_to(Vec3::new(0.0, -1.0, -1.0), Vec3::X),
    ));
}
