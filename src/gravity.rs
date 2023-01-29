use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_kira_audio::{AudioChannel, AudioControl};
use iyes_loopless::state::NextState;

use crate::{
    assets::GameAssets,
    audio::ForegroundAudio,
    game_state::GameState,
    gravity_spawner::{Deletable, Prediction, TrajectoryPoint},
    pickup::{ActivePickup, PickupType},
    player::{self, Player},
};

#[derive(Component)]
pub struct GravitationalBody(pub f32, pub f32);

#[derive(Component)]
pub struct DelayedActivity(pub f32);

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

pub const MAX_VELOCITY: f32 = 300.;

pub(crate) fn calculate_gravity(
    mut commands: Commands,
    query: Query<
        (
            Entity,
            &Transform,
            &GravitationTransform,
            &GravitationalBody,
        ),
        Without<DelayedActivity>,
    >,
    active_pickup: Res<ActivePickup>,
) {
    if matches!(active_pickup.0, Some(PickupType::Teleport)) {
        return;
    }
    for (entity, transform, velocity, gravity) in query.iter() {
        if let GravitationTransform::Velocity {
            velocity: v,
            start_position: _,
            target_position,
        } = velocity
        {
            let position = target_position.unwrap_or(transform.translation.xy());
            let (velocity, position, translation) =
                process_gravity_trajectory(v, position, query.iter(), entity, gravity, None);

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

pub fn process_gravity_trajectory<
    'a,
    T: Iterator<
        Item = (
            Entity,
            &'a Transform,
            &'a GravitationTransform,
            &'a GravitationalBody,
        ),
    >,
>(
    v: &Vec2,
    position: Vec2,
    query: T,
    entity: Entity,
    gravity: &GravitationalBody,
    phantom: Option<(&Vec2, &GravitationalBody)>,
) -> (Vec2, Vec2, Vec2) {
    let mut velocity = *v;
    for (entity_2, t_2, _, g_2) in query {
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

    if velocity.length() > MAX_VELOCITY {
        velocity = velocity.normalize() * MAX_VELOCITY;
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
    active_pickup: Res<ActivePickup>,
) {
    if matches!(active_pickup.0, Some(PickupType::Teleport)) {
        return;
    }
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
    query: Query<
        (
            Entity,
            &Transform,
            &GravitationTransform,
            &GravitationalBody,
        ),
        Without<Deletable>,
    >,
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
                                query.iter(),
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
    mut active_pickup: ResMut<ActivePickup>,
    audio: Res<AudioChannel<ForegroundAudio>>,
    assets: Res<GameAssets>,
) {
    if matches!(active_pickup.0, Some(PickupType::Teleport)) {
        return;
    }
    for player in players.iter() {
        for (entity, transforms, body) in gravitational_bodies.iter() {
            if player.translation.distance(transforms.translation) <= body.1 {
                audio.play(assets.destroyed_audio.clone());
                if matches!(active_pickup.0, Some(PickupType::PlanetKiller)) {
                    commands.entity(entity).despawn_recursive();
                    active_pickup.0 = None;
                } else {
                    commands.insert_resource(NextState(GameState::GameOver));
                }
            }
        }
    }
}

#[cfg(profile = "dev")]
pub fn gravity_bounding_visualizer(
    mut commands: Commands,
    bodies: Query<(Entity, &GravitationalBody), Changed<GravitationalBody>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    for (entity, body) in bodies.iter() {
        commands.entity(entity).despawn_descendants();
        commands.entity(entity).with_children(|p| {
            p.spawn(bevy::sprite::MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::RegularPolygon::new(body.1, 8).into())
                    .into(),
                material: color_materials.add(Color::ALICE_BLUE.into()),
                transform: Transform::from_translation(Vec3::new(0., 0., -5.)),
                ..default()
            });
        });
    }
}

pub fn set_sprite_to_radius(
    mut bodies: Query<(&mut Sprite, &GravitationalBody), Changed<GravitationalBody>>,
) {
    for (mut sprite, body) in bodies.iter_mut() {
        sprite.custom_size = Some(Vec2::ONE * 2. * body.1);
    }
}

pub const DELAYED_ACTIVITY_FLASH_DURATION: f32 = 0.3;

pub fn delayed_activity_flasher(
    mut bodies: Query<(Entity, &mut Visibility, &mut DelayedActivity)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    if bodies.is_empty() {
        return;
    }
    let division = time.elapsed_seconds() / DELAYED_ACTIVITY_FLASH_DURATION;
    let division = division - division.floor();
    let is_on = division > 0.5;
    let delta = time.delta_seconds();
    for (entity, mut visibility, mut delay) in bodies.iter_mut() {
        delay.0 -= delta;
        if delay.0 < 0. {
            commands.entity(entity).remove::<DelayedActivity>();
            visibility.is_visible = true;
        } else {
            visibility.is_visible = is_on;
        }
    }
}
