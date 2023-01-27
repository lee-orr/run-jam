use crate::level::LevelBoundary;
use crate::{gravity, player};
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

pub(crate) const BUFFER: f32 = 1200.0;

pub(crate) const VELOCITY_RATIO: f32 = 5.;

pub(crate) const VELOCITY_OFFSET_MAX: Vec2 = Vec2::splat(300.);

pub(crate) const VELOCITY_OFFSET_MIN: Vec2 = Vec2::splat(-300.);

const EDGE_DISPLAY_BUFFER: f32 = 50.;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct ElasticCentering(pub f32, pub Vec2);

pub(crate) type ElasticCamera<'a> = (
    &'a mut Transform,
    &'a mut OrthographicProjection,
    &'a mut ElasticCentering,
);

pub(crate) fn position_main_camera(
    mut camera: Query<ElasticCamera, (With<MainCamera>, Without<player::Player>)>,
    players: Query<(&Transform, &gravity::GravitationTransform), With<player::Player>>,
    time: Res<Time>,
    bounds: Res<LevelBoundary>,
) {
    let mut player_bounds = None;
    let mut target_offset = Vec2::ZERO;
    let mut num_players = 0.;
    for (player, velocity) in players.iter() {
        num_players += 1.;
        let pos = player.translation.xy();
        let offset = match velocity {
            gravity::GravitationTransform::Static => Vec2::ZERO,
            gravity::GravitationTransform::Velocity {
                velocity,
                start_position: _,
                target_position: _,
            } => *velocity * VELOCITY_RATIO,
        };
        let offset = offset.min(VELOCITY_OFFSET_MAX).max(VELOCITY_OFFSET_MIN);
        target_offset += offset;

        if let Some((min, max)) = player_bounds {
            player_bounds = Some((pos.min(min), pos.max(max)));
        } else {
            player_bounds = Some((pos, pos));
        }
    }

    if num_players > 0. {
        target_offset /= num_players;
    }

    let delta = time.delta_seconds();

    let camera_bounds = (
        bounds.min - EDGE_DISPLAY_BUFFER,
        bounds.max + EDGE_DISPLAY_BUFFER,
    );

    for (mut transform, mut projection, mut centering) in camera.iter_mut() {
        let camera_center = transform.translation.xy();

        let (gap, pos) = if let Some((min, max)) = player_bounds {
            let gap = (max - min) / 2.;
            (gap, gap + min)
        } else {
            (Vec2::ZERO, Vec2::ZERO)
        };

        let max_component = gap.max_element();
        let horizontal = BUFFER + max_component;

        let offset = (target_offset * delta + centering.1 * (centering.0 - delta)) / centering.0;
        centering.1 = offset;

        let pos = pos + offset;
        let target_dist = (pos - camera_center) * 2.;
        let pos = target_dist * delta + camera_center;

        let camera_bounds = (
            camera_bounds.0 - Vec2::new(projection.left, projection.bottom),
            camera_bounds.1 - Vec2::new(projection.right, projection.top),
        );
        let pos = pos.max(camera_bounds.0).min(camera_bounds.1);

        transform.translation = Vec3::new(pos.x, pos.y, 0.);

        projection.scaling_mode = ScalingMode::FixedHorizontal(horizontal);
    }
}
