use crate::player;
use bevy::prelude::*;



#[derive(Component)]
pub struct Goal(pub f32);

pub(crate) fn check_goal(
    mut commands: Commands,
    players: Query<&Transform, With<player::Player>>,
    goals: Query<(Entity, &Transform, &Goal)>,
) {
    for player in players.iter() {
        for (entity, goal_transform, goal_radius) in goals.iter() {
            if player.translation.distance(goal_transform.translation) <= goal_radius.0 {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
