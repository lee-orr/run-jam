use bevy::{math::Vec3Swizzles, prelude::*};
use iyes_loopless::state::NextState;

use crate::{
    game_state::GameState,
    gravity_spawner::{Prediction, TrajectoryPoint},
    player::{self, Player},
};

#[derive(Component)]
pub struct GravitationalBody(pub f32, pub f32);

#[derive(Component)]
pub enum GravitationTransform {
    Static,
    Velocity {
        velocity: Vec2,
        start_position: Option<Vec2>,
        target_position: Option<Vec2>,
    },
}

impl GravitationTransform {
    pub fn velocity(velocity: Vec2) -> Self {
        Self::Velocity {
            velocity,
            start_position: None,
            target_position: None,
        }
    }
}

pub(crate) const G: f32 = 30.;
pub(crate) const FIXED_TIME_FPS: f32 = 15.;
pub(crate) const FIXED_TIME_DELTA: f32 = 1. / FIXED_TIME_FPS;
pub(crate) const FIXED_TIME_MILIS: u64 = (FIXED_TIME_DELTA * 1000.) as u64;
pub const GAP_BETWEEN_TRAJECTORY: f32 = 0.5;

pub(crate) fn calculate_gravity(
    mut commands: Commands,
    query: Query<(
        Entity,
        &Transform,
        &GravitationTransform,
        &GravitationalBody,
    )>,
) {
    for (entity, transform, velocity, gravity) in query.iter() {
        if let GravitationTransform::Velocity {
            velocity: v,
            start_position: _,
            target_position,
        } = velocity
        {
            let position = target_position.unwrap_or(transform.translation.xy());
            let (velocity, position, translation) =
                process_gravity_trajectory(v, position, &query, entity, gravity, None);

            commands
                .entity(entity)
                .insert(GravitationTransform::Velocity {
                    velocity,
                    start_position: Some(position),
                    target_position: Some(translation),
                });
        }
    }
}

pub fn process_gravity_trajectory(
    v: &Vec2,
    position: Vec2,
    query: &Query<(
        Entity,
        &Transform,
        &GravitationTransform,
        &GravitationalBody,
    )>,
    entity: Entity,
    gravity: &GravitationalBody,
    phantom: Option<(&Vec2, &GravitationalBody)>,
) -> (Vec2, Vec2, Vec2) {
    let mut velocity = *v;
    for (entity_2, t_2, _, g_2) in query.iter() {
        if entity_2 == entity {
            continue;
        }
        let r = t_2.translation.xy() - position;
        let d_sq = r.length_squared();
        if d_sq > 30. {
            velocity += (G * FIXED_TIME_DELTA * gravity.0 * g_2.0 * r.normalize()) / (d_sq);
        }
    }
    if let Some((t_2, g_2)) = phantom {
        let r = *t_2 - position;
        let d_sq = r.length_squared();
        if d_sq > 30. {
            velocity += (G * FIXED_TIME_DELTA * gravity.0 * g_2.0 * r.normalize()) / (d_sq);
        }
    }
    let displacement = velocity * FIXED_TIME_DELTA;
    let translation = position + displacement;
    (velocity, position, translation)
}

pub(crate) fn adjust_rotation(mut query: Query<(Entity, &mut Transform, &GravitationTransform)>) {
    for (_entity, mut transform, gravitation_transform) in query.iter_mut() {
        if let GravitationTransform::Velocity {
            velocity,
            start_position: _,
            target_position: _,
        } = gravitation_transform
        {
            let angle = velocity.y.atan2(velocity.x);
            transform.rotation = Quat::from_axis_angle(Vec3::Z, angle);
        }
    }
}

pub(crate) fn smooth_movement(
    mut query: Query<(&mut Transform, &GravitationTransform)>,
    time: Res<Time>,
) {
    let delta = time.delta_seconds();
    let proportion = delta / FIXED_TIME_DELTA;
    for (mut transform, gravitation_transform) in query.iter_mut() {
        if let GravitationTransform::Velocity {
            velocity: _,
            start_position,
            target_position,
        } = gravitation_transform
        {
            if let (Some(start), Some(end)) = (start_position, target_position) {
                let diff = *end - *start;
                let diff = Vec3::new(diff.x, diff.y, 0.);
                transform.translation += diff * proportion;
            }
        }
    }
}

type TrajectoryQueryConditions = (With<TrajectoryPoint>, Without<GravitationalBody>);

pub(crate) fn predict_trajectory(
    mut trajectory_points: Query<(&mut Transform, &mut Visibility), TrajectoryQueryConditions>,
    query: Query<(
        Entity,
        &Transform,
        &GravitationTransform,
        &GravitationalBody,
    )>,
    player: Query<Entity, With<Player>>,
    prediction: Res<Prediction>,
) {
    let prediction = prediction.as_ref();
    if let Ok(player) = player.get_single() {
        if let Ok((
            entity,
            transform,
            GravitationTransform::Velocity {
                velocity,
                start_position: _,
                target_position,
            },
            grav_body,
        )) = query.get(player)
        {
            match prediction {
                Prediction::Insert(prediction_pos, grav) => {
                    let mut trajectory_pos = *(target_position
                        .as_ref()
                        .unwrap_or(&transform.translation.xy()));
                    let mut v = *velocity;
                    for (mut t, mut vis) in trajectory_points.iter_mut() {
                        let mut dist = 0.;
                        loop {
                            dist += FIXED_TIME_DELTA;
                            if dist >= GAP_BETWEEN_TRAJECTORY {
                                break;
                            }
                            let (vel, _, p) = process_gravity_trajectory(
                                &v,
                                trajectory_pos,
                                &query,
                                entity,
                                grav_body,
                                Some((prediction_pos, grav)),
                            );
                            trajectory_pos = p;
                            v = vel;
                        }

                        vis.is_visible = true;
                        t.translation = Vec3::new(trajectory_pos.x, trajectory_pos.y, 0.);
                    }
                }
                _ => {
                    for (_, mut v) in trajectory_points.iter_mut() {
                        v.is_visible = false;
                    }
                }
            }
        }
    }
}

pub fn check_crash(
    mut commands: Commands,
    players: Query<&Transform, With<player::Player>>,
    gravitational_bodies: Query<(Entity, &Transform, &GravitationalBody), Without<player::Player>>,
) {
    for player in players.iter() {
        for (_, transforms, body) in gravitational_bodies.iter() {
            if player.translation.distance(transforms.translation) <= body.1 {
                commands.insert_resource(NextState(GameState::GameOver));
            }
        }
    }
}
