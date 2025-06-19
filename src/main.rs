mod app;

use app::*;

use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb_u8(23, 147, 209))) // #1793d1: Arch Linux
        .add_systems(Startup, (spawn_camera, spawn_ground, spawn_light))
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "dash-crush-rush".into(),
                    ..default()
                }),
                ..default()
            }),
            GamePlugin,
        ))
        .run();
}
