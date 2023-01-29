use bevy::prelude::*;

use crate::{
    assets::GameAssets,
    pickup::{ActivePickup, PickupType},
};
#[derive(Component)]
pub struct Player;

pub fn set_player_image(
    mut commands: Commands,
    players: Query<Entity, With<Player>>,
    active_pickup: Res<ActivePickup>,
    assets: Res<GameAssets>,
) {
    if !active_pickup.is_changed() {
        return;
    }

    let image = match active_pickup.0 {
        Some(PickupType::PlanetKiller) => assets.player_planet_killer.clone(),
        Some(PickupType::Teleport) => assets.player_teleport.clone(),
        _ => assets.player.clone(),
    };

    for player in players.iter() {
        commands.entity(player).insert(image.clone());
    }
}

pub const PLANET_KILLER_FLASH_DURATION: f32 = 0.3;

pub fn player_has_pickup_modifiers(
    mut players: Query<&mut Sprite, With<Player>>,
    active_pickup: Res<ActivePickup>,
    time: Res<Time>,
) {
    match active_pickup.0 {
        Some(PickupType::PlanetKiller) => {
            let division = time.elapsed_seconds() / PLANET_KILLER_FLASH_DURATION;
            let division = division - division.floor();
            let is_on = division > 0.5;

            for mut player in players.iter_mut() {
                player.color = if is_on {
                    Color::rgb(0.8, 0.8, 0.8)
                } else {
                    Color::WHITE
                };
            }
        }
        _ => {
            for mut player in players.iter_mut() {
                player.color = Color::WHITE;
            }
        }
    }
}
