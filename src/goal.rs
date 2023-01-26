use crate::player;
use bevy::prelude::*;

#[derive(Resource)]
pub struct Score(pub usize);

impl Score {
    pub fn value(&self) -> usize {
        self.0
    }
}

#[derive(Component)]
pub struct Goal(pub f32);

pub enum GoalEvent {
    Collected,
}

pub(crate) fn check_goal(
    mut commands: Commands,
    players: Query<&Transform, With<player::Player>>,
    goals: Query<(Entity, &Transform, &Goal)>,
    mut score: ResMut<Score>,
    mut events: EventWriter<GoalEvent>,
) {
    for player in players.iter() {
        for (entity, goal_transform, goal_radius) in goals.iter() {
            if player.translation.distance(goal_transform.translation) <= goal_radius.0 {
                commands.entity(entity).despawn_recursive();
                score.0 += 1;
                events.send(GoalEvent::Collected)
            }
        }
    }
}
