use crate::{
    assets::GameAssets,
    audio::ForegroundAudio,
    level::{GoalStatus, LevelEvent},
    player,
};
use bevy::prelude::*;
use bevy_kira_audio::{AudioChannel, AudioControl};

#[derive(Component)]
pub struct Pickup(pub f32, pub PickupType);

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PickupType {
    Goal,
    PlanetKiller,
    Teleport,
}

#[derive(Resource)]
pub struct ActivePickup(pub Option<PickupType>);

pub const PICKUPS: [PickupType; 2] = [PickupType::Teleport, PickupType::PlanetKiller];

pub(crate) fn check_pickup(
    mut commands: Commands,
    players: Query<&Transform, With<player::Player>>,
    goals: Query<(Entity, &Transform, &Pickup)>,
    mut goal_status: ResMut<GoalStatus>,
    mut events: EventWriter<LevelEvent>,
    mut active_pickup: ResMut<ActivePickup>,
    audio: Res<AudioChannel<ForegroundAudio>>,
    assets: Res<GameAssets>,
) {
    if matches!(active_pickup.0, Some(PickupType::Teleport)) {
        return;
    }

    for player in players.iter() {
        for (entity, transform, pickup) in goals.iter() {
            if player.translation.distance(transform.translation) <= pickup.0 {
                commands.entity(entity).despawn_recursive();
                if pickup.1 == PickupType::Goal {
                    let goal_type = goal_status.current;
                    goal_status.completed.push(goal_type);
                    audio.play(assets.collected_audio.clone());
                } else {
                    active_pickup.0 = Some(pickup.1);
                    audio.play(assets.pickup_audio.clone());
                }
                events.send(LevelEvent::PickupCollected(pickup.1))
            }
        }
    }
}
