use crate::{
    level::{GoalStatus, LevelEvent},
    player,
};
use bevy::prelude::*;

#[derive(Component)]
pub struct Pickup(pub f32, pub PickupType);

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PickupType {
    Goal,
    PlanetKiller,
}

#[derive(Resource)]
pub struct ActivePickup(pub Option<PickupType>);

pub const PICKUPS: [PickupType; 1] = [PickupType::PlanetKiller];

pub(crate) fn check_pickup(
    mut commands: Commands,
    players: Query<&Transform, With<player::Player>>,
    goals: Query<(Entity, &Transform, &Pickup)>,
    mut goal_status: ResMut<GoalStatus>,
    mut events: EventWriter<LevelEvent>,
    mut active_pickup: ResMut<ActivePickup>,
) {
    for player in players.iter() {
        for (entity, transform, pickup) in goals.iter() {
            if player.translation.distance(transform.translation) <= pickup.0 {
                commands.entity(entity).despawn_recursive();
                if pickup.1 == PickupType::Goal {
                    let goal_type = goal_status.current;
                    goal_status.completed.push(goal_type);
                } else {
                    active_pickup.0 = Some(pickup.1);
                }
                events.send(LevelEvent::PickupCollected(pickup.1))
            }
        }
    }
}
