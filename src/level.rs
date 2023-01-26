use crate::{
    assets::GameAssets, game_state::GameState, goal, gravity, player, space_material::SpaceMaterial,
};
use bevy::{math::Vec3Swizzles, prelude::*};
use iyes_loopless::state::NextState;

#[derive(Resource)]
pub struct LevelBoundary {
    pub(crate) min: Vec2,
    pub(crate) max: Vec2,
}

#[derive(Component)]
pub struct Backdrop;

#[derive(Component)]
pub struct LevelEntity;

pub(crate) fn check_boundary(
    players: Query<&Transform, With<player::Player>>,
    boundary: Res<LevelBoundary>,
    mut commands: Commands,
) {
    for player in players.iter() {
        if player.translation.xy().cmplt(boundary.min).any()
            || player.translation.xy().cmpgt(boundary.max).any()
        {
            commands.insert_resource(NextState(GameState::GameOver));
            return;
        }
    }
}

pub(crate) fn update_backdrop(
    boundary: Res<LevelBoundary>,
    backdrop: Query<Entity, With<Backdrop>>,
    mut commands: Commands,
    mut materials: ResMut<Assets<SpaceMaterial>>,
) {
    if !boundary.is_changed() {
        return;
    }

    let material = materials.add(SpaceMaterial {
        map_boundary: Vec4::new(
            boundary.min.x,
            boundary.min.y,
            boundary.max.x,
            boundary.max.y,
        ),
        ..default()
    });

    for backdrop in backdrop.iter() {
        commands.entity(backdrop).insert(material.clone());
    }
}

pub fn start_level(mut commands: Commands, assets: Res<GameAssets>) {
    commands
        .spawn((SpatialBundle::default(), LevelEntity))
        .with_children(|p| {
            p.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::ONE * 50.),
                        ..Default::default()
                    },
                    texture: assets.large_planet.clone(),
                    transform: Transform::from_translation(Vec3::new(0., -100., 0.)),
                    ..default()
                },
                gravity::GravitationalBody(10000., 50.),
                gravity::GravitationTransform::Static,
            ));

            p.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::ONE * 50.),
                        ..Default::default()
                    },
                    texture: assets.goal.clone(),
                    transform: Transform::from_translation(Vec3::new(0., 50., 0.)),
                    ..default()
                },
                goal::Goal(30.),
            ));

            p.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::ONE * 50.),
                        ..Default::default()
                    },
                    texture: assets.player.clone(),
                    transform: Transform::from_translation(Vec3::new(-500., 0., 0.)),
                    ..default()
                },
                gravity::GravitationalBody(1., 10.),
                player::Player,
                gravity::GravitationTransform::velocity(Vec2::X * 50.),
            ));
        });
}

pub fn clear_level(
    mut commands: Commands,
    levels: Query<Entity, With<LevelEntity>>,
    mut cameras: Query<&mut Transform, With<Camera>>,
) {
    for level in levels.iter() {
        commands.entity(level).despawn_recursive();
    }

    for mut camera in cameras.iter_mut() {
        camera.translation = Vec3::ZERO;
    }
}
