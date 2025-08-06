mod app;

use app::*;

use bevy::prelude::*;

fn main() {
    App::new()
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
