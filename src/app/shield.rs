use super::*;

use bevy::prelude::*;

pub struct ShieldPlugin;

impl Plugin for ShieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ShieldPlayer>()
            .add_systems(Update, (shield_player, color_player));
    }
}

#[derive(Component)]
pub struct Shield {
    pub active: bool,
}

#[derive(Event)]
struct ShieldPlayer(Color);

fn shield_player(
    key: Res<ButtonInput<MouseButton>>,
    mut shield: Single<&mut Shield, With<Player>>,
    mut active: EventWriter<ShieldPlayer>,
) {
    if key.pressed(cfg::bind::SHIELD) {
        shield.active = true;
        active.write(ShieldPlayer(cfg::player::shield::COLOR));
    } else {
        shield.active = false;
        active.write(ShieldPlayer(cfg::player::default::COLOR));
    }
}

fn color_player(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut material: Single<&mut MeshMaterial3d<StandardMaterial>, With<Player>>,
    mut active: EventReader<ShieldPlayer>,
) {
    for active in active.read() {
        material.0 = materials.add(active.0);
    }
}
