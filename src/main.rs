mod app;

use app::*;

use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb_u8(23, 147, 209))) // #1793d1: Arch Linux
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "dash-crush-rush".into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (spawn_camera, spawn_ground, spawn_player))
        .add_systems(Update, move_player)
        .run();
}
