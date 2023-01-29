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
}

pub(crate) fn check_pickup(
    mut commands: Commands,
    players: Query<&Transform, With<player::Player>>,
    goals: Query<(Entity, &Transform, &Pickup)>,
    mut goal_status: ResMut<GoalStatus>,
    mut events: EventWriter<LevelEvent>,
) {
    for player in players.iter() {
        for (entity, transform, pickup) in goals.iter() {
            if player.translation.distance(transform.translation) <= pickup.0 {
                commands.entity(entity).despawn_recursive();
                if pickup.1 == PickupType::Goal {
                    let goal_type = goal_status.current;
                    goal_status.completed.push(goal_type);
                }
                events.send(LevelEvent::PickupCollected(pickup.1))
            }
        }
    }
}
