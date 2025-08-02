use super::*;

use bevy::prelude::*;

pub struct DestroyPlugin;

impl Plugin for DestroyPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Destroy>()
            .add_systems(Update, destroy_player);
    }
}

#[derive(Event)]
pub struct Destroy(pub Reason);

pub enum Reason {
    Enemy,
}

fn destroy_player(
    mut destroy: EventWriter<Destroy>,
    player: Single<&Transform, With<Player>>,
    enemies: Query<&Transform, With<Enemy>>,
) {
    for enemy in enemies {
        let distance = player.translation.distance(enemy.translation);
        if distance <= cfg::player::default::RADIUS + cfg::enemy::RADIUS {
            destroy.write(Destroy(Reason::Enemy));
        }
    }
}
